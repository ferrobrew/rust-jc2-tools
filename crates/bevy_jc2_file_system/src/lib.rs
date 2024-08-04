use async_lock::RwLock;
use bevy::{
    asset::{
        io::{AssetSource, AssetSourceId},
        AssetLoadFailedEvent,
    },
    prelude::*,
    utils::HashMap,
};
use jc2_hashing::HashString;
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

mod archive;
#[cfg(feature = "tree")]
use archive::ArchivePaths;
use archive::{archive_type, Archive, ArchiveLoader, ArchiveType};

mod asset_reader;
use asset_reader::FileSystemAssetReader;

#[cfg(feature = "tree")]
mod tree;
#[cfg(feature = "tree")]
pub use tree::{FileSystemTree, FileSystemTreeIter, FileSystemTreeIterValue};

#[cfg(feature = "tree")]
mod file_list;
#[cfg(feature = "tree")]
use file_list::{FileList, FileListLoader};

#[derive(Default, Debug)]
pub struct FileSystemPlugin;

#[derive(Event, Debug)]
pub enum FileSystemEvent {
    DirectoryMounted { path: PathBuf },
    DirectoryUnmounted { path: PathBuf },
    ArchivePending { path: PathBuf },
    ArchiveMounted { path: PathBuf },
    ArchiveUnmounted { path: PathBuf },
    ArchiveError { path: PathBuf },
}

#[derive(Default, Debug)]
pub struct FileSystemMountsData {
    pub(crate) pending_archives: AtomicUsize,
    pub(crate) pending_stream_archives: AtomicUsize,
    pub(crate) directories: RwLock<Vec<PathBuf>>,
    pub(crate) archives: RwLock<Vec<Archive>>,
}

#[derive(Resource, Default, Debug)]
pub struct FileSystemMounts {
    pub(crate) mounts: Arc<FileSystemMountsData>,
    pub(crate) pending_archives: HashMap<HashString, Handle<Archive>>,
    pub(crate) pending_events: Vec<FileSystemEvent>,
    #[cfg(feature = "tree")]
    pub file_tree: FileSystemTree,
}

impl FileSystemMounts {
    pub fn mount_directory(&mut self, path: impl AsRef<Path>) -> &Self {
        let path: PathBuf = path.as_ref().into();
        {
            let mut directories = self.mounts.directories.write_blocking();
            let directory_count = directories.len();
            directories.retain(|directory| *directory != path);
            directories.push(path.clone());
            if directories.len() > directory_count {
                self.pending_events
                    .push(FileSystemEvent::DirectoryMounted { path });
            }
        }
        self
    }

    pub fn unmount_directory(&mut self, path: impl AsRef<Path>) -> &Self {
        let path: PathBuf = path.as_ref().into();

        // Attempt to unmount the archive, and validate success
        {
            let mut directories = self.mounts.directories.write_blocking();
            let directory_count = directories.len();
            directories.retain(|directory| directory != &path);
            if directories.len() == directory_count {
                return self;
            }
        }

        self.pending_events
            .push(FileSystemEvent::DirectoryUnmounted { path: path.clone() });

        // We must now unmount corresponding archives, leaving streaming archives alone
        {
            let mut archives = self.mounts.archives.write_blocking();
            archives.retain(|archive| {
                // We skip archives without a `target_path`
                let Some(target_path) = &archive.target_path else {
                    return true;
                };

                // This is not bullet-proof, as multiple directories could contain `target_path`
                // Better to be safe than sorry, and it's *very* unlikely this will ever occur
                let unmount = path.join(target_path).is_file();
                if unmount {
                    self.pending_events.push(FileSystemEvent::ArchiveUnmounted {
                        path: target_path.clone(),
                    });
                }
                unmount
            });
        }

        self
    }

    pub fn mount_archive(&mut self, asset_server: &AssetServer, path: impl AsRef<Path>) -> &Self {
        let path: PathBuf = path.as_ref().into();
        let hash = HashString::from_str(&path.to_string_lossy());

        // Already pending, and can be skipped
        if self.pending_archives.contains_key(&hash) {
            return self;
        }

        // Already mounted, and can be skipped
        if self.has_mounted_archive(path.clone()) {
            return self;
        }

        // All pending archives count towards total
        match archive_type(&path) {
            ArchiveType::Stream => {
                self.mounts
                    .pending_stream_archives
                    .fetch_add(1, Ordering::Relaxed);
            }
            ArchiveType::File => {
                self.mounts.pending_archives.fetch_add(1, Ordering::Relaxed);
            }
            ArchiveType::Unknown => {}
        }

        self.pending_archives
            .insert(hash, asset_server.load(path.clone()));
        self.pending_events
            .push(FileSystemEvent::ArchivePending { path });
        self
    }

    pub fn unmount_archive(&mut self, path: impl AsRef<Path>) -> &Self {
        let path: PathBuf = path.as_ref().into();
        let hash = HashString::from_str(&path.to_string_lossy());

        // Unmount the archive, if it's already mounted
        {
            let mut archives = self.mounts.archives.write_blocking();
            let archive_count = archives.len();
            #[cfg(not(feature = "tree"))]
            archives.retain(|archive| archive.hash != hash);
            #[cfg(feature = "tree")]
            archives.retain_mut(|archive| {
                let retain = archive.hash != hash;
                if !retain {
                    match &mut archive.paths {
                        ArchivePaths::FileList(_) => {
                            self.file_tree.remove(&path);
                        }
                        ArchivePaths::HashList(paths) => {
                            for file in std::mem::take(paths)
                                .into_values()
                                .filter_map(|e| e.into_path())
                            {
                                self.file_tree.remove(&path.join(&file));
                            }
                        }
                    }
                    self.file_tree.sort();
                }
                retain
            });
            if archives.len() < archive_count {
                self.pending_events
                    .push(FileSystemEvent::ArchiveUnmounted { path: path.clone() });
            }
        }

        self.pending_archives.remove(&hash);
        self
    }

    pub fn is_mounting_archives(&self) -> bool {
        !self.pending_archives.is_empty()
    }

    pub fn is_mounting_archive(&self, path: impl Into<PathBuf>) -> bool {
        let path: PathBuf = path.into();
        let hash = HashString::from_str(&path.to_string_lossy());
        self.pending_archives.contains_key(&hash)
    }

    pub fn has_mounted_archive(&self, path: impl Into<PathBuf>) -> bool {
        let path: PathBuf = path.into();
        let hash = HashString::from_str(&path.to_string_lossy());
        self.mounts
            .archives
            .read_blocking()
            .iter()
            .any(|archive| archive.hash == hash)
    }
}

impl Plugin for FileSystemPlugin {
    fn build(&self, app: &mut App) {
        let mounts = Arc::new(FileSystemMountsData::default());
        app.insert_resource(FileSystemMounts {
            mounts: mounts.clone(),
            #[cfg(feature = "tree")]
            file_tree: FileSystemTree::with_capacity(256),
            ..default()
        })
        .register_asset_source(
            AssetSourceId::Default,
            AssetSource::build().with_reader(move || {
                Box::new(FileSystemAssetReader::new(
                    mounts.clone(),
                    AssetSource::get_default_reader("assets".into()),
                ))
            }),
        )
        .add_event::<FileSystemEvent>()
        .add_systems(First, process_archive_events);
    }

    fn finish(&self, app: &mut App) {
        app.init_asset::<Archive>()
            .register_asset_loader(ArchiveLoader);

        #[cfg(feature = "tree")]
        {
            app.init_asset::<FileList>()
                .register_asset_loader(FileListLoader);
        }
    }
}

fn process_archive_events(
    #[cfg(feature = "tree")] file_lists: Res<Assets<FileList>>,
    #[cfg(feature = "tree")] mut failed_file_lists: EventReader<AssetLoadFailedEvent<FileList>>,
    mut archives: ResMut<Assets<Archive>>,
    mut event_writer: EventWriter<FileSystemEvent>,
    mut failed_archives: EventReader<AssetLoadFailedEvent<Archive>>,
    mut load_events: EventReader<AssetEvent<Archive>>,
    mut mounts: ResMut<FileSystemMounts>,
) {
    // Process pending events
    for event in mounts.pending_events.drain(..) {
        event_writer.send(event);
    }

    let mut processed_stream_archives = 0usize;
    let mut processed_archives = 0usize;
    let mut process = |mounts: &mut FileSystemMounts, path: &Path, error: bool| {
        match archive_type(path) {
            ArchiveType::Stream => processed_stream_archives += 1,
            ArchiveType::File => processed_archives += 1,
            ArchiveType::Unknown => {}
        };
        event_writer.send(if error {
            FileSystemEvent::ArchiveError { path: path.into() }
        } else {
            FileSystemEvent::ArchiveMounted { path: path.into() }
        });
        mounts
            .pending_archives
            .remove(&HashString::from_str(&path.to_string_lossy()));
    };

    // Process loaded archives
    for archive in load_events
        .read()
        .filter_map(|event| match event {
            AssetEvent::LoadedWithDependencies { id } => Some(*id),
            _ => None,
        })
        .filter_map(|h| archives.remove(h))
    {
        let hash = archive.hash;

        // Validate that the archive load wasn't cancelled
        if !mounts.pending_archives.contains_key(&hash) {
            process(mounts.as_mut(), &archive.source_path, true);
            continue;
        };

        // Validate that the `target_path` still exists
        if let Some(target_path) = &archive.target_path {
            let exists = mounts
                .mounts
                .directories
                .read_blocking()
                .iter()
                .any(|directory| directory.join(target_path).is_file());
            if !exists {
                process(mounts.as_mut(), &archive.source_path, true);
                continue;
            }
        };

        // Finally mount the archive, and emit mounted event
        let path = archive.source_path.clone();

        #[cfg(feature = "tree")]
        {
            let Some(paths) = (match &archive.paths {
                ArchivePaths::FileList(handle) => file_lists.get(handle).map(|list| &list.paths),
                ArchivePaths::HashList(paths) => Some(paths),
            }) else {
                process(mounts.as_mut(), &archive.source_path, true);
                continue;
            };

            for hash in archive.entries.keys() {
                mounts.file_tree.insert(&match paths.find_path(*hash) {
                    Some(path) => archive.source_path.join(path),
                    None => archive
                        .source_path
                        .join(&PathBuf::from(hash.hash().to_string())),
                });
            }
            mounts.file_tree.sort();
        }

        mounts.mounts.archives.write_blocking().push(archive);
        process(mounts.as_mut(), &path, false);
    }

    // Process failed file lists
    #[cfg(feature = "tree")]
    for file_list_handle in failed_file_lists.read().map(|event| event.id) {
        for (_, archive) in archives.iter() {
            match &archive.paths {
                ArchivePaths::FileList(file_list) if file_list.id() == file_list_handle => {
                    process(mounts.as_mut(), &archive.source_path, true);
                }
                _ => {}
            }
        }
    }

    // Process failed archives
    for path in failed_archives.read().map(|event| &event.path) {
        process(mounts.as_mut(), path.path(), true);
    }

    // Handle processed archives
    mounts
        .mounts
        .pending_stream_archives
        .fetch_sub(processed_stream_archives, Ordering::Relaxed);
    mounts
        .mounts
        .pending_archives
        .fetch_sub(processed_archives, Ordering::Relaxed);
}

use bevy::{
    asset::{
        io::{AssetSource, AssetSourceId},
        AssetLoadFailedEvent,
    },
    prelude::*,
    utils::HashMap,
};
use jc2_hashing::HashString;
use parking_lot::RwLock;
use std::{path::PathBuf, sync::Arc};

mod archive;
use archive::{Archive, ArchiveLoader};

mod asset_reader;
use asset_reader::FileSystemAssetReader;

#[derive(Default, Debug)]
pub struct FileSystemPlugin;

#[derive(Default, Debug)]
pub struct FileSystemMountsData {
    pub(crate) directories: RwLock<Vec<PathBuf>>,
    pub(crate) archives: RwLock<Vec<Archive>>,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct FileSystemMounts {
    pub(crate) mounts: Arc<FileSystemMountsData>,
    pub(crate) pending_archives: HashMap<HashString, Handle<Archive>>,
}

impl FileSystemMounts {
    pub fn mount_directory(&self, path: impl Into<PathBuf>) -> &Self {
        let directory = path.into();
        let mut directories = self.mounts.directories.write();
        directories.retain(|d| *d != directory);
        directories.push(directory);
        self
    }

    pub fn unmount_directory(&self, path: impl Into<PathBuf>) -> &Self {
        let directory: PathBuf = path.into();
        self.mounts.directories.write().retain(|d| *d != directory);
        self
    }

    pub fn mount_archive(&mut self, asset_server: &AssetServer, path: impl Into<PathBuf>) -> &Self {
        let path: PathBuf = path.into();
        let hash = HashString::from_str(&path.to_string_lossy());
        for archive in self.mounts.archives.read().iter() {
            if archive.hash == hash {
                return self;
            }
        }
        self.pending_archives.insert(hash, asset_server.load(path));
        self
    }

    pub fn unmount_archive(&mut self, path: impl Into<PathBuf>) -> &Self {
        let path: PathBuf = path.into();
        let hash = HashString::from_str(&path.to_string_lossy());
        self.mounts
            .archives
            .write()
            .retain(|archive| archive.hash != hash);
        self.pending_archives.remove(&hash);
        self
    }
}

impl Plugin for FileSystemPlugin {
    fn build(&self, app: &mut App) {
        let mounts = Arc::new(FileSystemMountsData::default());
        app.insert_resource(FileSystemMounts {
            mounts: mounts.clone(),
            pending_archives: HashMap::new(),
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
        .add_systems(First, process_archive_events);
    }

    fn finish(&self, app: &mut App) {
        app.init_asset::<Archive>()
            .register_asset_loader(ArchiveLoader);
    }
}

fn process_archive_events(
    mut archives: ResMut<Assets<Archive>>,
    mut load_events: EventReader<AssetEvent<Archive>>,
    mut failed_events: EventReader<AssetLoadFailedEvent<Archive>>,
    mut mounts: ResMut<FileSystemMounts>,
) {
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
        mounts.mounts.archives.write().push(archive);
        mounts.pending_archives.remove(&hash);
    }

    // Process failed archives
    for hash in failed_events
        .read()
        .map(|event| HashString::from_str(&event.path.to_string()))
    {
        mounts.pending_archives.remove(&hash);
    }
}

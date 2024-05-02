use bevy::{
    asset::{
        io::{AssetSource, AssetSourceId},
        AssetPath,
    },
    prelude::*,
};
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
    pub(crate) pending_archives: Vec<(PathBuf, Handle<Archive>)>,
}

impl FileSystemMounts {
    pub fn mount_directory(&self, path: impl Into<PathBuf>) -> &Self {
        self.mounts.directories.write().push(path.into());
        self
    }

    pub fn unmount_directory(&self, path: impl Into<PathBuf>) -> &Self {
        let directory: PathBuf = path.into();
        self.mounts.directories.write().retain(|d| *d != directory);
        self
    }

    pub fn mount_archive<'a>(
        &mut self,
        asset_server: &AssetServer,
        path: impl Into<PathBuf> + Into<AssetPath<'a>> + Clone,
    ) -> &Self {
        let entry = (path.clone().into(), asset_server.load(path));
        self.pending_archives.push(entry);
        self
    }

    pub fn unmount_archive(&mut self, path: impl Into<PathBuf>) -> &Self {
        let path = path.into();
        self.pending_archives.retain(|(p, _)| *p != path);
        self
    }
}

impl Plugin for FileSystemPlugin {
    fn build(&self, app: &mut App) {
        let mounts = Arc::new(FileSystemMountsData::default());
        app.insert_resource(FileSystemMounts {
            mounts: mounts.clone(),
            pending_archives: vec![],
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
            .preregister_asset_loader::<ArchiveLoader>(&["rbm"])
            .register_asset_loader(ArchiveLoader);
    }
}

fn process_archive_events(
    mut archives: ResMut<Assets<Archive>>,
    mut events: EventReader<AssetEvent<Archive>>,
    mut mounts: ResMut<FileSystemMounts>,
) {
    for archive in events
        .read()
        .filter_map(|e| match e {
            AssetEvent::LoadedWithDependencies { id } => Some(*id),
            _ => None,
        })
        .filter_map(|h| archives.remove(h))
    {
        mounts.pending_archives.retain(|(p, _)| *p != archive.path);
        let mut archives = mounts.mounts.archives.write();
        archives.retain(|a| a.path != archive.path);
        archives.push(archive);
    }
}

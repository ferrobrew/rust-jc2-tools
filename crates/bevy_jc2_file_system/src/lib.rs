use bevy::{
    asset::io::{AssetSource, AssetSourceId},
    prelude::{App, AssetApp, Plugin, Resource},
};
use parking_lot::RwLock;
use std::{path::PathBuf, sync::Arc};

mod asset_reader;
pub use asset_reader::FileSystemAssetReader;

#[derive(Default, Debug)]
pub struct FileSystemPlugin;

#[derive(Default, Debug)]
pub struct FileSystemMountsData {
    pub(crate) directories: RwLock<Vec<PathBuf>>,
    pub(crate) archives: RwLock<Vec<PathBuf>>,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct FileSystemMounts {
    pub(crate) mounts: Arc<FileSystemMountsData>,
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

    pub fn mount_archive(&self, path: impl Into<PathBuf>) -> &Self {
        self.mounts.archives.write().push(path.into());
        self
    }

    pub fn unmount_archive(&self, path: impl Into<PathBuf>) -> &Self {
        let archive: PathBuf = path.into();
        self.mounts.archives.write().retain(|a| *a != archive);
        self
    }
}

impl Plugin for FileSystemPlugin {
    fn build(&self, app: &mut App) {
        let mounts = Arc::new(FileSystemMountsData::default());
        app.insert_resource(FileSystemMounts {
            mounts: mounts.clone(),
        });
        app.register_asset_source(
            AssetSourceId::Default,
            AssetSource::build().with_reader(move || {
                Box::new(FileSystemAssetReader::new(
                    mounts.clone(),
                    AssetSource::get_default_reader("assets".into()),
                ))
            }),
        );
    }
}

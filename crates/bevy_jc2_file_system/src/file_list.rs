use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use jc2_hashing::HashList;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum FileListError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("hash collision: {path:?}")]
    HashCollision { path: String },
}

#[derive(Asset, Debug, Clone, TypePath)]
pub(crate) struct FileList {
    pub(crate) paths: HashList,
}

#[derive(Default)]
pub(crate) struct FileListLoader;

impl AssetLoader for FileListLoader {
    type Asset = FileList;
    type Settings = ();
    type Error = FileListError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut buffer = String::new();
            reader.read_to_string(&mut buffer).await?;

            let mut paths = HashList::new();
            for path in buffer.lines() {
                if paths.insert_path(path).is_some() {
                    return Err(FileListError::HashCollision { path: path.into() });
                }
            }

            Ok(FileList { paths })
        })
    }
}

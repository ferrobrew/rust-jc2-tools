use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use jc2_file_formats::archive::{ArchiveTable, ArchiveTableEntry};
use jc2_hashing::HashString;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArchiveError {
    #[error("invalid rbm file: {0}")]
    Binrw(#[from] binrw::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Asset, Debug, Clone, TypePath)]
pub struct Archive {
    pub(crate) path: PathBuf,
    pub(crate) entries: Arc<HashMap<HashString, ArchiveTableEntry>>,
}

#[derive(Default)]
pub struct ArchiveLoader;

impl AssetLoader for ArchiveLoader {
    type Asset = Archive;
    type Settings = ();
    type Error = ArchiveError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let archive = ArchiveTable::read(&mut binrw::io::Cursor::new(&bytes))?;
            let table_path: PathBuf = load_context.asset_path().to_owned().into();
            let archive_path = table_path.with_extension("arc");

            Ok(Archive {
                path: archive_path,
                entries: Arc::new(archive.entries),
            })
        })
    }
}

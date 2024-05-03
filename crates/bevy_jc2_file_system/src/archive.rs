use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use jc2_file_formats::archive::{ArchiveTable, ArchiveTableEntry, StreamArchive};
use jc2_hashing::HashString;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArchiveError {
    #[error("invalid file: {0}")]
    Binrw(#[from] binrw::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unknown format: {extension:?}")]
    UnknownFormat { extension: Option<String> },
}

#[derive(Debug, Clone)]
pub enum ArchiveEntry {
    Streamed(ArchiveTableEntry),
    Preloaded(Vec<u8>),
}

#[derive(Asset, Debug, Clone, TypePath)]
pub struct Archive {
    pub(crate) path: PathBuf,
    pub(crate) entries: Arc<HashMap<HashString, ArchiveEntry>>,
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
            let mut cursor = binrw::io::Cursor::new(&bytes);

            match load_context.path().extension().and_then(OsStr::to_str) {
                Some("tab") => {
                    let archive = ArchiveTable::read(&mut cursor)?;
                    let table_path: PathBuf = load_context.asset_path().to_owned().into();
                    let archive_path = table_path.with_extension("arc");
                    let entries = archive
                        .entries
                        .into_iter()
                        .map(|(k, v)| (k, ArchiveEntry::Streamed(v)))
                        .collect();

                    Ok(Archive {
                        path: archive_path,
                        entries: Arc::new(entries),
                    })
                }
                Some("ee") => {
                    let archive = StreamArchive::read(&mut cursor)?;
                    let entries = archive
                        .entries
                        .into_iter()
                        .map(|(k, v)| (HashString::from_str(&k), ArchiveEntry::Preloaded(v)))
                        .collect();

                    Ok(Archive {
                        path: load_context.path().into(),
                        entries: Arc::new(entries),
                    })
                }
                Some(extension) => Err(ArchiveError::UnknownFormat {
                    extension: Some(extension.into()),
                }),
                None => Err(ArchiveError::UnknownFormat { extension: None }),
            }
        })
    }
}

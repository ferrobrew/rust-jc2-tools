use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use jc2_file_formats::archive::{ArchiveTable, ArchiveTableEntry, StreamArchive};
use jc2_hashing::HashString;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ArchiveError {
    #[error("invalid file: {0}")]
    Binrw(#[from] binrw::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unknown format: {extension:?}")]
    UnknownFormat { extension: Option<String> },
}

#[derive(Debug, Clone)]
pub(crate) enum ArchiveEntry {
    Streamed(ArchiveTableEntry),
    Preloaded(Vec<u8>),
}

#[derive(Asset, Debug, Clone, TypePath)]
pub(crate) struct Archive {
    pub(crate) hash: HashString,
    pub(crate) path: PathBuf,
    pub(crate) entries: Arc<HashMap<HashString, ArchiveEntry>>,
}

#[derive(Default)]
pub(crate) struct ArchiveLoader;

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
                    Ok(Archive {
                        hash: HashString::from_str(&load_context.path().to_string_lossy()),
                        path: load_context.path().with_extension("arc"),
                        entries: Arc::new(
                            archive
                                .entries
                                .into_iter()
                                .map(|(k, v)| (k, ArchiveEntry::Streamed(v)))
                                .collect(),
                        ),
                    })
                }
                Some("ee") => {
                    let archive = StreamArchive::read(&mut cursor)?;
                    Ok(Archive {
                        hash: HashString::from_str(&load_context.path().to_string_lossy()),
                        path: load_context.path().into(),
                        entries: Arc::new(
                            archive
                                .entries
                                .into_iter()
                                .map(|(k, v)| {
                                    (HashString::from_str(&k), ArchiveEntry::Preloaded(v))
                                })
                                .collect(),
                        ),
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

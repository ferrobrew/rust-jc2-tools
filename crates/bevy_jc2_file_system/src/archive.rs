use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use jc2_file_formats::archive::{ArchiveTable, ArchiveTableEntry, StreamArchive};
#[cfg(feature = "tree")]
use jc2_hashing::HashList;
use jc2_hashing::HashString;
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[cfg(feature = "tree")]
use crate::file_list::FileList;

#[derive(Error, Debug)]
pub(crate) enum ArchiveError {
    #[error("invalid file: {0}")]
    Binrw(#[from] binrw::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unknown format: {path:?}")]
    UnknownFormat { path: PathBuf },
}

#[derive(Debug, Clone)]
pub(crate) enum ArchiveEntry {
    Streamed(ArchiveTableEntry),
    Preloaded(Vec<u8>),
}

#[cfg(feature = "tree")]
#[derive(Debug, Clone)]
pub(crate) enum ArchivePaths {
    FileList(Handle<FileList>),
    HashList(HashList),
}

#[derive(Asset, Debug, Clone, TypePath)]
pub(crate) struct Archive {
    pub(crate) hash: HashString,
    pub(crate) source_path: PathBuf,
    pub(crate) target_path: Option<PathBuf>,
    #[cfg(feature = "tree")]
    pub(crate) paths: ArchivePaths,
    pub(crate) entries: HashMap<HashString, ArchiveEntry>,
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

            let source_path = load_context.path().to_path_buf();
            let hash = HashString::from_str(&source_path.to_string_lossy());

            match archive_type(load_context.path()) {
                ArchiveType::Stream => {
                    let archive = StreamArchive::read(&mut cursor)?;
                    Ok(Archive {
                        hash,
                        source_path,
                        target_path: None,
                        #[cfg(feature = "tree")]
                        paths: ArchivePaths::HashList({
                            let mut paths = HashList::with_capacity(archive.entries.len());
                            for path in archive.entries.keys() {
                                paths.insert_path(path);
                            }
                            paths
                        }),
                        entries: archive
                            .entries
                            .into_iter()
                            .map(|(k, v)| (HashString::from_str(&k), ArchiveEntry::Preloaded(v)))
                            .collect(),
                    })
                }
                ArchiveType::File => {
                    let archive = ArchiveTable::read(&mut cursor)?;
                    Ok(Archive {
                        hash,
                        source_path: source_path.clone(),
                        target_path: Some(source_path.with_extension("arc")),
                        #[cfg(feature = "tree")]
                        paths: ArchivePaths::FileList(
                            load_context.load(source_path.with_extension("filelist")),
                        ),
                        entries: archive
                            .entries
                            .into_iter()
                            .map(|(k, v)| (k, ArchiveEntry::Streamed(v)))
                            .collect(),
                    })
                }
                ArchiveType::Unknown => Err(ArchiveError::UnknownFormat { path: source_path }),
            }
        })
    }
}

pub(crate) enum ArchiveType {
    Stream,
    File,
    Unknown,
}

pub(crate) fn archive_type(path: &Path) -> ArchiveType {
    match path.extension().and_then(OsStr::to_str) {
        Some("tab") => ArchiveType::File,
        Some("ee") => ArchiveType::Stream,
        _ => ArchiveType::Unknown,
    }
}

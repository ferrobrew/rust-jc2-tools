use std::{
    fs::{read_dir, File},
    io::{Cursor, Read, Seek},
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    task::Poll,
};

use bevy::asset::io::{AssetReader, AssetReaderError, PathStream, Reader};
use futures_io::{AsyncRead, AsyncSeek};
use futures_lite::Stream;
use jc2_hashing::HashString;

use crate::{archive::ArchiveEntry, FileSystemMountsData};

pub(crate) struct FileSystemAssetReader {
    mounts: Arc<FileSystemMountsData>,
    default_reader: Box<dyn AssetReader>,
}

impl std::fmt::Debug for FileSystemAssetReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FilesystemAssetReader")
            .field("data", &self.mounts)
            .finish_non_exhaustive()
    }
}

impl FileSystemAssetReader {
    #[must_use]
    pub(crate) fn new(
        mounts: Arc<FileSystemMountsData>,
        mut reader: impl FnMut() -> Box<dyn AssetReader> + Send + Sync + 'static,
    ) -> Self {
        Self {
            mounts,
            default_reader: reader(),
        }
    }

    fn is_file(&self, path: &Path) -> bool {
        // Check mounted directories for our file, using the full path
        for directory in self.mounts.directories.read().iter() {
            let file = directory.join(path);
            if file.is_file() {
                return true;
            }
        }

        // Check mounted archives for our file, using only the name
        if let Some(hash) = path
            .file_name()
            .map(|name| HashString::from_bytes(name.as_encoded_bytes()))
        {
            for archive in self.mounts.archives.read().iter() {
                if archive.entries.contains_key(&hash) {
                    return true;
                }
            }
        }

        // Nothing found
        false
    }

    fn read_file(&self, path: &Path) -> Result<FileReader, AssetReaderError> {
        // Attempt to get a reader from mounted directories, using the full path
        for directory in self.mounts.directories.read().iter() {
            let file = directory.join(path);
            if file.is_file() {
                return Ok(FileReader::from(File::open(file)?));
            }
        }

        // Attempt to get a reader from mounted archives, using only the name
        if let Some(hash) = path
            .file_name()
            .map(|name| HashString::from_bytes(name.as_encoded_bytes()))
        {
            for archive in self.mounts.archives.read().iter() {
                // Skip archives that don't contain our hash
                let Some(entry) = archive.entries.get(&hash) else {
                    continue;
                };

                let buffer = match &entry {
                    ArchiveEntry::Streamed(streamed) => {
                        // Get the archive path from mounted directories
                        let Some(path) =
                            self.mounts.directories.read().iter().find_map(|directory| {
                                let path = directory.join(&archive.path);
                                path.is_file().then_some(path)
                            })
                        else {
                            continue;
                        };

                        // Open the archive, read the file, and create a cursor
                        let mut file = File::open(path)?;
                        file.seek(std::io::SeekFrom::Start(streamed.offset as u64))?;
                        let mut buffer = vec![0u8; streamed.size as usize];
                        file.read_exact(&mut buffer)?;
                        buffer
                    }
                    ArchiveEntry::Preloaded(buffer) => buffer.clone(),
                };

                return Ok(FileReader::from(Cursor::new(buffer)));
            }
        }

        // Nothing found
        Err(AssetReaderError::NotFound(path.into()))
    }

    fn is_directory(&self, path: &Path) -> bool {
        let folder = path.join("");
        for directory in self.mounts.directories.read().iter() {
            let file = directory.join(folder.clone());
            if file.is_dir() {
                return true;
            }
        }
        false
    }

    fn read_directory(&self, path: &Path) -> Result<DirReader, AssetReaderError> {
        if self.is_directory(path) {
            let mut paths = Vec::new();
            for directory in self.mounts.directories.read().iter() {
                if let Ok(read_dir) = read_dir(&directory.join(path)) {
                    let root_path = directory.clone();
                    let mapped_stream = read_dir.filter_map(move |f| {
                        f.ok().and_then(|dir_entry| {
                            let path = dir_entry.path();
                            // filter out meta files as they are not considered assets
                            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                if ext.eq_ignore_ascii_case("meta") {
                                    return None;
                                }
                            }
                            // Should never fail
                            if let Ok(relative_path) = path.strip_prefix(&root_path) {
                                Some(relative_path.to_owned())
                            } else {
                                None
                            }
                        })
                    });
                    paths.extend(mapped_stream);
                }
            }
            Ok(DirReader(paths))
        } else {
            Err(AssetReaderError::NotFound(path.to_path_buf()))
        }
    }
}

enum FileReaderValue {
    File(File),
    Cursor(Cursor<Vec<u8>>),
}

struct FileReader(FileReaderValue);

impl From<File> for FileReader {
    fn from(value: File) -> Self {
        Self(FileReaderValue::File(value))
    }
}

impl From<Cursor<Vec<u8>>> for FileReader {
    fn from(value: Cursor<Vec<u8>>) -> Self {
        Self(FileReaderValue::Cursor(value))
    }
}

impl AsyncRead for FileReader {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        Poll::Ready(match &mut self.get_mut().0 {
            FileReaderValue::File(file) => file.read(buf),
            FileReaderValue::Cursor(cursor) => cursor.read(buf),
        })
    }
}

impl AsyncSeek for FileReader {
    fn poll_seek(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        pos: std::io::SeekFrom,
    ) -> Poll<std::io::Result<u64>> {
        Poll::Ready(match &mut self.get_mut().0 {
            FileReaderValue::File(file) => file.seek(pos),
            FileReaderValue::Cursor(cursor) => cursor.seek(pos),
        })
    }
}

struct DirReader(Vec<PathBuf>);

impl Stream for DirReader {
    type Item = PathBuf;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        Poll::Ready(this.0.pop())
    }
}

pub(crate) fn get_meta_path(path: &Path) -> PathBuf {
    let mut meta_path = path.to_path_buf();
    let mut extension = path
        .extension()
        .expect("asset paths must have extensions")
        .to_os_string();
    extension.push(".meta");
    meta_path.set_extension(extension);
    meta_path
}

impl AssetReader for FileSystemAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> bevy::utils::BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        if self.is_file(path) {
            Box::pin(async move {
                self.read_file(path).map(|reader| {
                    let boxed: Box<Reader> = Box::new(reader);
                    boxed
                })
            })
        } else {
            self.default_reader.read(path)
        }
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> bevy::utils::BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        let meta_path = get_meta_path(path);
        if self.is_file(&meta_path) {
            Box::pin(async move {
                self.read_file(&meta_path).map(|reader| {
                    let boxed: Box<Reader> = Box::new(reader);
                    boxed
                })
            })
        } else {
            self.default_reader.read_meta(path)
        }
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> bevy::utils::BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        if self.is_directory(path) {
            Box::pin(async move {
                self.read_directory(path).map(|read_dir| {
                    let boxed: Box<PathStream> = Box::new(read_dir);
                    boxed
                })
            })
        } else {
            self.default_reader.read_directory(path)
        }
    }

    fn is_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> bevy::utils::BoxedFuture<'a, Result<bool, AssetReaderError>> {
        if self.is_directory(path) {
            Box::pin(async move { Ok(true) })
        } else {
            self.default_reader.is_directory(path)
        }
    }
}

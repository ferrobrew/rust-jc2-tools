use std::{
    path::{Path, PathBuf},
    pin::Pin,
    sync::{Arc, atomic::Ordering},
    task::Poll,
};

use async_fs::{File, read_dir};
use bevy::asset::io::{AssetReader, AssetReaderError, ErasedAssetReader, PathStream, Reader};
use futures_io::{AsyncRead, AsyncSeek, SeekFrom};
use futures_lite::{AsyncReadExt, AsyncSeekExt, Stream, StreamExt, future::yield_now, io::Cursor};
use jc2_hashing::HashString;

use crate::{
    FileSystemMountsData,
    archive::{ArchiveEntry, ArchiveType, archive_type},
};

pub(crate) struct FileSystemAssetReader {
    mounts: Arc<FileSystemMountsData>,
    default_reader: Box<dyn ErasedAssetReader>,
}

impl std::fmt::Debug for FileSystemAssetReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FilesystemAssetReader")
            .field("mounts", &self.mounts)
            .finish_non_exhaustive()
    }
}

impl FileSystemAssetReader {
    #[must_use]
    pub(crate) fn new(
        mounts: Arc<FileSystemMountsData>,
        mut reader: impl FnMut() -> Box<dyn ErasedAssetReader> + Send + Sync + 'static,
    ) -> Self {
        Self {
            mounts,
            default_reader: reader(),
        }
    }

    async fn read(&self, path: &Path) -> Result<FileReader, AssetReaderError> {
        // Attempt to get a reader from mounted directories, using the full path
        for directory in self.mounts.directories.read().await.iter() {
            let file = directory.join(path);
            if file.is_file() {
                return Ok(FileReader::from(File::open(file).await?));
            }
        }

        // If the file does not come from a mounted directory, then we must wait for archives
        match archive_type(path) {
            ArchiveType::Stream => {
                while self.mounts.pending_archives.load(Ordering::Relaxed) > 0 {
                    yield_now().await;
                }
            }
            ArchiveType::Unknown => {
                // Select file types can bypass gates, yes, really
                let bypass = path
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("filelist"));
                if !bypass {
                    while self.mounts.pending_archives.load(Ordering::Relaxed) > 0 {
                        yield_now().await;
                    }
                    while self.mounts.pending_stream_archives.load(Ordering::Relaxed) > 0 {
                        yield_now().await;
                    }
                }
            }
            ArchiveType::File => {}
        }

        // Attempt to get a reader from mounted archives, using only the name
        if let Some(hash) = path
            .file_name()
            .map(|name| HashString::from_bytes(name.as_encoded_bytes()))
        {
            for archive in self.mounts.archives.read().await.iter() {
                // Skip archives that don't contain our hash
                let Some(entry) = archive.entries.get(&hash) else {
                    continue;
                };

                let buffer = match &entry {
                    ArchiveEntry::Streamed(streamed) => {
                        // If the target path is not set, something went very wrong
                        let Some(target_path) = &archive.target_path else {
                            unreachable!("`archive.target_path` was not set!");
                        };

                        // Get the archive path from mounted directories
                        let Some(path) =
                            self.mounts
                                .directories
                                .read()
                                .await
                                .iter()
                                .find_map(|directory| {
                                    let path = directory.join(target_path);
                                    path.is_file().then_some(path)
                                })
                        else {
                            // Currently impossible to use bevy_assets::get_base_path() as a fallback...
                            continue;
                        };

                        // Open the archive, read the file, and create a cursor
                        let mut file = File::open(path).await?;
                        file.seek(SeekFrom::Start(streamed.offset as u64)).await?;
                        let mut buffer = vec![0u8; streamed.size as usize];
                        file.read_exact(&mut buffer).await?;
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

    async fn is_directory(&self, path: &Path) -> bool {
        let folder = path.join("");
        for directory in self.mounts.directories.read().await.iter() {
            let file = directory.join(folder.clone());
            if file.is_dir() {
                return true;
            }
        }
        false
    }

    async fn read_directory(&self, path: &Path) -> Result<DirReader, AssetReaderError> {
        if self.is_directory(path).await {
            let mut paths = Vec::new();
            for directory in self.mounts.directories.read().await.iter() {
                if let Ok(read_dir) = read_dir(&directory.join(path)).await {
                    let root_path = directory.clone();
                    let mapped_stream: Vec<PathBuf> = read_dir
                        .filter_map(|f| {
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
                        })
                        .collect()
                        .await;
                    paths.extend(mapped_stream);
                }
            }
            Ok(DirReader(paths))
        } else {
            Err(AssetReaderError::NotFound(path.to_path_buf()))
        }
    }
}

struct FileReader<'a>(Box<Reader<'a>>);

impl<'a> From<File> for FileReader<'a> {
    fn from(value: File) -> Self {
        Self(Box::new(value))
    }
}

impl<'a> From<Cursor<Vec<u8>>> for FileReader<'a> {
    fn from(value: Cursor<Vec<u8>>) -> Self {
        Self(Box::new(value))
    }
}

impl<'a> AsyncRead for FileReader<'a> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        std::pin::pin!(&mut self.get_mut().0).poll_read(cx, buf)
    }
}

impl<'a> AsyncSeek for FileReader<'a> {
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        pos: SeekFrom,
    ) -> Poll<std::io::Result<u64>> {
        std::pin::pin!(&mut self.get_mut().0).poll_seek(cx, pos)
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
    async fn read<'a>(&'a self, path: &'a Path) -> Result<Box<Reader<'a>>, AssetReaderError> {
        match self.read(path).await {
            Ok(reader) => Ok(Box::new(reader) as Box<Reader>),
            Err(_) => self.default_reader.read(path).await,
        }
    }

    async fn read_meta<'a>(&'a self, path: &'a Path) -> Result<Box<Reader<'a>>, AssetReaderError> {
        match self.read(&get_meta_path(path)).await {
            Ok(reader) => Ok(Box::new(reader) as Box<Reader>),
            Err(_) => self.default_reader.read_meta(path).await,
        }
    }

    async fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        if self.is_directory(path).await {
            self.read_directory(path)
                .await
                .map(|read_dir| Box::new(read_dir) as Box<PathStream>)
        } else {
            self.default_reader.read_directory(path).await
        }
    }

    async fn is_directory<'a>(&'a self, path: &'a Path) -> Result<bool, AssetReaderError> {
        if self.is_directory(path).await {
            Ok(true)
        } else {
            self.default_reader.is_directory(path).await
        }
    }
}

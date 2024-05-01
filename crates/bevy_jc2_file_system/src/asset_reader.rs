use std::{
    fs::{read_dir, File},
    io::{Read, Seek},
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    task::Poll,
};

use bevy::asset::io::{AssetReader, AssetReaderError, PathStream, Reader};
use futures_io::{AsyncRead, AsyncSeek};
use futures_lite::Stream;

use crate::FileSystemMountsData;

pub struct FileSystemAssetReader {
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
        for directory in self.mounts.directories.read().iter() {
            let file = directory.join(path);
            if file.is_file() {
                return true;
            }
        }
        false
    }

    fn read_file(&self, path: &Path) -> Result<FileReader, AssetReaderError> {
        for directory in self.mounts.directories.read().iter() {
            let file = directory.join(path);
            if file.is_file() {
                return Ok(FileReader(File::open(file)?));
            }
        }
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

struct FileReader(File);

impl AsyncRead for FileReader {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let read = this.0.read(buf);
        Poll::Ready(read)
    }
}

impl AsyncSeek for FileReader {
    fn poll_seek(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        pos: std::io::SeekFrom,
    ) -> Poll<std::io::Result<u64>> {
        let this = self.get_mut();
        let seek = this.0.seek(pos);
        Poll::Ready(seek)
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

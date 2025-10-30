use std::{collections::HashMap, thread};

use godot::{
    classes::{DirAccess, Engine, FileAccess, file_access::ModeFlags, notify::ObjectNotification},
    global::Error as GodotError,
    prelude::*,
    task,
};
use jc2_file_formats::archive::{ArchiveTable, StreamArchive};
use jc2_hashing::HashString;
use thiserror::Error;

mod formats;
use formats::JcResourceFormats;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct JcResourceLoader {
    base: Base<Object>,
    channel: Option<JcResourceChannel>,
    callbacks: HashMap<GString, Vec<Callable>>,
}

#[godot_api]
impl JcResourceLoader {
    const NAME: &str = "JcResourceLoader";

    pub fn singleton() -> Gd<Self> {
        // TODO: can we make this return GdMut?...
        Engine::singleton()
            .get_singleton(Self::NAME)
            .expect(&format!("failed to get {}", Self::NAME))
            .cast::<Self>()
    }

    pub fn register() {
        Engine::singleton().register_singleton(Self::NAME, &Self::new_alloc());
    }

    pub fn unregister() {
        let singleton = Self::singleton();
        Engine::singleton().unregister_singleton(Self::NAME);
        singleton.free();
    }

    fn send(&self, task: JcResourceTask) -> bool {
        let result = match &self.channel {
            Some(channel) => channel.sender.send_blocking(task),
            None => Err(async_channel::SendError(task)),
        };

        if let Err(task) = &result {
            godot_error!("{}: failed to send task {task:?}", Self::NAME);
        }

        result.is_ok()
    }

    fn receive(&mut self, event: JcResourceEvent) {
        match event {
            JcResourceEvent::ResourceLoaded(path, resource) => {
                if let Some(callbacks) = self.callbacks.remove(&path) {
                    let args = [path.to_variant(), resource.0.to_variant()];
                    for callback in callbacks {
                        callback.call(&args);
                    }
                } else if !resource.0.instance_id().is_ref_counted() {
                    resource.0.free();
                }
            }
            JcResourceEvent::ResourceError(path, error) => {
                self.callbacks.remove(&path);
                godot_error!("{}: failed to load resource ({error}) {path}", Self::NAME);
            }
            JcResourceEvent::DirectoryMounted(path) => {
                godot_print!("{}: Directory mounted {path}", Self::NAME);
            }
            JcResourceEvent::DirectoryUnmounted(path) => {
                godot_print!("{}: Directory unmounted {path}", Self::NAME);
            }
            JcResourceEvent::ArchiveMounted(path) => {
                godot_print!("{}: Archive mounted {path}", Self::NAME);
            }
            JcResourceEvent::ArchiveUnmounted(path) => {
                godot_print!("{}: Archive unmounted {path}", Self::NAME);
            }
            JcResourceEvent::StreamArchiveMounted(path) => {
                godot_print!("{}: Stream archive mounted {path}", Self::NAME);
            }
            JcResourceEvent::StreamArchiveUnmounted(path) => {
                godot_print!("{}: Stream archive unmounted {path}", Self::NAME);
            }
        }
    }

    #[func]
    pub fn mount_directory(&mut self, path: GString) {
        self.send(JcResourceTask::MountDirectory(path));
    }

    #[func]
    pub fn unmount_directory(&mut self) {
        self.send(JcResourceTask::UnmountDirectory);
    }

    #[func]
    pub fn mount_stream_archive(&mut self, path: GString) {
        self.send(JcResourceTask::MountStreamArchive(path));
    }

    #[func]
    pub fn unmount_stream_archive(&mut self, path: GString) {
        self.send(JcResourceTask::UnmountStreamArchive(path));
    }

    #[func]
    pub fn load_resource(&mut self, path: GString, callback: Callable) {
        if self.send(JcResourceTask::LoadResource(path.clone())) {
            if let Some(callbacks) = self.callbacks.get_mut(&path) {
                callbacks.push(callback);
            } else {
                self.callbacks.insert(path, vec![callback]);
            }
        }
    }
}

#[godot_api]
impl IObject for JcResourceLoader {
    fn on_notification(&mut self, what: ObjectNotification) {
        match what {
            ObjectNotification::POSTINITIALIZE => {
                self.channel = JcResourceChannel::new(self.base.to_init_gd().cast());
            }
            _ => {}
        }
    }
}

struct JcResourceChannel {
    sender: async_channel::Sender<JcResourceTask>,
    handles: Option<(thread::JoinHandle<()>, task::TaskHandle)>,
}

impl JcResourceChannel {
    pub fn new(mut owner: Gd<JcResourceLoader>) -> Option<Self> {
        let (task_sender, task_receiver) = async_channel::bounded(64 * 64 * 2);
        let (result_sender, result_receiver) = async_channel::bounded(64 * 64 * 2);

        let Some(thread) = JcResourceThread::spawn(task_receiver, result_sender) else {
            godot_error!("JcResourceChannel: failed to spawn thread");
            return None;
        };

        let task = task::spawn(async move {
            godot_print!("JcResourceChannel: task started");
            loop {
                let Ok(event) = result_receiver.recv().await else {
                    break;
                };
                owner.bind_mut().receive(event);
            }
            godot_print!("JcResourceChannel: task stopped");
        });

        Some(Self {
            sender: task_sender,
            handles: Some((thread, task)),
        })
    }
}

impl Drop for JcResourceChannel {
    fn drop(&mut self) {
        if let Some((thread, task)) = std::mem::take(&mut self.handles) {
            task.cancel();
            godot_print!("JcResourceChannel: task cancelled");

            if self.sender.send_blocking(JcResourceTask::Shutdown).is_ok() {
                if thread.join().is_err() {
                    godot_warn!("JcResourceChannel: resource thread panicked");
                }
            } else {
                godot_warn!("JcResourceChannel: failed to send shutdown task");
            }
        };
    }
}

#[derive(Debug)]
struct JcResourceThread {
    directory: GString,
    archives: Vec<(ArchiveTable, Gd<FileAccess>)>,
    stream_archives: Vec<(StreamArchive, GString)>,
    formats: JcResourceFormats,
    events: Vec<JcResourceEvent>,
}

impl JcResourceThread {
    pub fn spawn(
        receiver: async_channel::Receiver<JcResourceTask>,
        sender: async_channel::Sender<JcResourceEvent>,
    ) -> Option<thread::JoinHandle<()>> {
        thread::Builder::new()
            .spawn(move || {
                JcResourceThread {
                    directory: Default::default(),
                    archives: Default::default(),
                    stream_archives: Default::default(),
                    events: Default::default(),
                    formats: formats::register(),
                }
                .run(receiver, sender);
            })
            .ok()
    }

    fn run(
        &mut self,
        receiver: async_channel::Receiver<JcResourceTask>,
        sender: async_channel::Sender<JcResourceEvent>,
    ) {
        godot_print!("JcResourceThead: thread started");
        while let Ok(task) = receiver.recv_blocking() {
            let result = match task {
                JcResourceTask::MountDirectory(path) => self.mount_directory(path),
                JcResourceTask::UnmountDirectory => self.unmount_directory(),
                JcResourceTask::MountStreamArchive(path) => self.mount_stream_archive(path),
                JcResourceTask::UnmountStreamArchive(path) => self.unmount_stream_archive(path),
                JcResourceTask::LoadResource(path) => self.load_resource(path),
                JcResourceTask::Shutdown => {
                    break;
                }
            };

            match result {
                Ok(()) => {
                    for event in std::mem::take(&mut self.events) {
                        if sender.send_blocking(event).is_err() {
                            godot_error!("JcResourceThread: failed to send event");
                            break;
                        }
                    }
                }
                Err(error) => {
                    godot_error!("{error:?}");
                }
            }
        }
        godot_print!("JcResourceThead: thread stopped");
    }

    fn mount_directory(&mut self, path: GString) -> JcResourceResult<()> {
        if path == self.directory {
            return Ok(());
        }

        let mut archives = vec![];
        Self::load_directory(path.path_join("archives_win32"), &mut archives)?;
        Self::load_directory(path.path_join("DLC"), &mut archives)?;
        self.unmount_directory()?;

        self.archives = archives;
        self.events.reserve(self.archives.len() + 1);
        for (_, file) in &self.archives {
            self.events
                .push(JcResourceEvent::ArchiveMounted(file.get_path()));
        }

        self.directory = path;
        self.events
            .push(JcResourceEvent::DirectoryMounted(self.directory.clone()));

        Ok(())
    }

    fn unmount_directory(&mut self) -> JcResourceResult<()> {
        if self.directory.is_empty() {
            return Ok(());
        }

        self.events.reserve(self.archives.len() + 1);
        for (_, file) in std::mem::take(&mut self.archives) {
            self.events
                .push(JcResourceEvent::ArchiveUnmounted(file.get_path()));
        }
        self.events
            .push(JcResourceEvent::DirectoryUnmounted(std::mem::take(
                &mut self.directory,
            )));

        Ok(())
    }

    fn mount_stream_archive(&mut self, path: GString) -> JcResourceResult<()> {
        if self
            .stream_archives
            .iter()
            .find(|stream_archive| stream_archive.1 == path)
            .is_some()
        {
            return Ok(());
        }

        let archive = self.load_stream_archive(path.clone())?;
        self.stream_archives.push((archive, path.clone()));
        self.events
            .push(JcResourceEvent::StreamArchiveMounted(path));

        Ok(())
    }

    fn unmount_stream_archive(&mut self, path: GString) -> JcResourceResult<()> {
        let Some(index) = self
            .stream_archives
            .iter()
            .position(|stream_archive| stream_archive.1 == path)
        else {
            return Err(JcResourceError::FileAccess {
                path,
                error: GodotError::ERR_FILE_NOT_FOUND,
            });
        };

        self.stream_archives.swap_remove(index);
        self.events
            .push(JcResourceEvent::StreamArchiveUnmounted(path));
        Ok(())
    }

    fn load_resource(&mut self, path: GString) -> JcResourceResult<()> {
        let event = match self.create_resource(path.clone()) {
            Ok(resource) => JcResourceEvent::ResourceLoaded(path, JcResource(resource)),
            Err(error) => JcResourceEvent::ResourceError(path, error),
        };
        self.events.push(event);
        Ok(())
    }

    fn load_directory(
        path: GString,
        archives: &mut Vec<(ArchiveTable, Gd<FileAccess>)>,
    ) -> JcResourceResult<()> {
        let Some(mut directory) = DirAccess::open(&path) else {
            let error = DirAccess::get_open_error();
            return Err(JcResourceError::DirectoryAccess { path, error });
        };

        let files = directory.get_files();
        let filtered_files: Vec<&GString> = {
            let mut result: Vec<&GString> = files
                .as_slice()
                .iter()
                .filter(|file| file.ends_with(".tab"))
                .collect();
            result.sort_by_key(|file| file.len());
            result
        };

        for file in filtered_files {
            if !file.ends_with(".tab") {
                continue;
            }

            archives.push(Self::load_archive(path.path_join(file))?);
        }

        Ok(())
    }

    fn load_archive(path: GString) -> JcResourceResult<(ArchiveTable, Gd<FileAccess>)> {
        let table = FileAccess::get_file_as_bytes(&path);
        if table.is_empty() {
            let error = FileAccess::get_open_error();
            return Err(JcResourceError::FileAccess { path, error });
        }

        let path = path.replace(".tab", ".arc");
        let Some(file) = FileAccess::open(&path, ModeFlags::READ) else {
            let error = FileAccess::get_open_error();
            return Err(JcResourceError::FileAccess { path, error });
        };

        let mut cursor = binrw::io::Cursor::new(table.as_slice());
        match ArchiveTable::read(&mut cursor) {
            Ok(table) => Ok((table, file)),
            Err(error) => Err(JcResourceError::Binrw { path, error }),
        }
    }

    fn load_stream_archive(&self, path: GString) -> JcResourceResult<StreamArchive> {
        let buffer = self.get_buffer(&path)?;
        let mut cursor = binrw::io::Cursor::new(buffer.as_slice());

        // TODO: convert entries to Map<HashString, Entry> + Arc<Vec<GString>>?
        match StreamArchive::read(&mut cursor) {
            Ok(archive) => Ok(archive),
            Err(error) => Err(JcResourceError::Binrw { path, error }),
        }
    }

    fn create_resource(&mut self, path: GString) -> JcResourceResult<Gd<Object>> {
        let extension = path.get_extension().to_lower();
        let Some(loader) = self.formats.get(&extension) else {
            return Err(JcResourceError::FileAccess {
                path,
                error: GodotError::ERR_INVALID_PARAMETER,
            });
        };

        let buffer = self.get_buffer(&path)?;
        loader(path, buffer, self)
    }

    fn get_buffer(&self, path: &GString) -> JcResourceResult<PackedByteArray> {
        let file = path.get_file().to_lower().to_string();
        let result = self.stream_archives.iter().find_map(|archive| {
            archive
                .0
                .entries
                .get(&file)
                .map(|entry| PackedByteArray::from(entry.clone()))
        });

        if let Some(buffer) = result {
            Ok(buffer)
        } else {
            let hash = HashString::from_str(&file);
            let result = self
                .archives
                .iter()
                .map(|archive| (&archive.0.entries, &archive.1))
                .find_map(|(entries, archive)| {
                    entries.get(&hash).map(|entry| {
                        let mut archive = archive.clone();
                        archive.seek(entry.offset as u64);
                        archive.get_buffer(entry.size as i64)
                    })
                });

            if let Some(buffer) = result {
                Ok(buffer)
            } else {
                Err(JcResourceError::FileAccess {
                    path: path.clone(),
                    error: GodotError::ERR_FILE_NOT_FOUND,
                })
            }
        }
    }
}

type JcResourceResult<T> = Result<T, JcResourceError>;

#[derive(Error, Debug)]
enum JcResourceError {
    #[error("directory access error ({path:?}): {error:?}")]
    DirectoryAccess { path: GString, error: GodotError },
    #[error("file access error ({path:?}): {error:?}")]
    FileAccess { path: GString, error: GodotError },
    #[error("binrw error: {path:?}")]
    Binrw { path: GString, error: binrw::Error },
}

#[derive(Debug)]
enum JcResourceTask {
    MountDirectory(GString),
    UnmountDirectory,
    MountStreamArchive(GString),
    UnmountStreamArchive(GString),
    LoadResource(GString),
    Shutdown,
}

#[derive(Debug)]
enum JcResourceEvent {
    DirectoryMounted(GString),
    DirectoryUnmounted(GString),
    ArchiveMounted(GString),
    ArchiveUnmounted(GString),
    StreamArchiveMounted(GString),
    StreamArchiveUnmounted(GString),
    ResourceLoaded(GString, JcResource),
    ResourceError(GString, JcResourceError),
}

#[derive(Debug)]
struct JcResource(Gd<Object>);

impl From<Gd<Object>> for JcResource {
    fn from(value: Gd<Object>) -> Self {
        Self(value)
    }
}

/// SAFETY: `Resource` is safe to create on any thread, and to send to main thread.
unsafe impl Send for JcResource {}

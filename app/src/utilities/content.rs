use bevy::prelude::*;
use bevy_file_dialog::DialogDirectoryPicked;
use bevy_jc2_file_system::{FileSystemEvent, FileSystemMounts};
use itertools::Itertools;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ContentDirectory>()
            .init_resource::<ContentDirectory>()
            .init_state::<ContentState>()
            .add_systems(
                Last,
                (
                    pick_content_directory,
                    mount_content_directory,
                    update_content_state,
                )
                    .chain(),
            );
    }
}

#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct ContentDirectory {
    pub target: Option<PathBuf>,
    previous: Option<PathBuf>,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentState {
    #[default]
    Unloaded,
    Loaded,
}

fn pick_content_directory(
    mut content: ResMut<ContentDirectory>,
    mut events: EventReader<DialogDirectoryPicked<ContentDirectory>>,
) {
    for path in events.read().map(|e| e.path.clone()) {
        content.target = Some(path);
    }
}

fn mount_content_directory(
    asset_server: Res<AssetServer>,
    mut directory: ResMut<ContentDirectory>,
    mut mounts: ResMut<FileSystemMounts>,
) {
    if directory.is_changed() {
        // No need to do anything
        if directory.target == directory.previous {
            return;
        }

        // Clean up previously mounted directory
        if let Some(previous) = &directory.previous {
            mounts.unmount_directory(previous);
            directory.previous = None;
        }

        // Mount the target directory
        if let Some(target) = &directory.target {
            mounts.mount_directory(target);

            // Discover archives
            let archives = ["archives_win32", "DLC"]
                .iter()
                .filter_map(|directory| std::fs::read_dir(target.join(directory)).ok())
                .flat_map(|files| {
                    files
                        .filter_map(|file| file.ok().map(|f| f.path()))
                        .filter(|file| file.extension().is_some_and(|f| f == "tab"))
                        .sorted()
                })
                .collect::<Vec<PathBuf>>();

            // Mount discovered archives
            for archive in archives
                .iter()
                .filter_map(|archive| archive.strip_prefix(target).ok())
            {
                mounts.mount_archive(&asset_server, archive);
            }

            directory.previous = Some(target.clone());
        }
    }
}

fn update_content_state(
    directory: Res<ContentDirectory>,
    mut events: EventReader<FileSystemEvent>,
    mut next_state: ResMut<NextState<ContentState>>,
) {
    for event in events.read() {
        if let Some(directory) = &directory.target {
            match event {
                FileSystemEvent::DirectoryMounted { path } => {
                    if path == directory {
                        next_state.set(ContentState::Loaded);
                    }
                }
                FileSystemEvent::DirectoryUnmounted { path } => {
                    if path == directory {
                        next_state.set(ContentState::Unloaded);
                    }
                }
                _ => {}
            }
        }
    }
}

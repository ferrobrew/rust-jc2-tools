use bevy::{asset::AssetPath, prelude::*};
use bevy_file_dialog::DialogFilePicked;
use bevy_jc2_file_system::FileSystemMounts;
use bevy_jc2_render_block::{RenderBlockBundle, RenderBlockMesh};

use crate::utilities::content::ContentSet;

#[derive(Debug, Default)]
pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TargetArchive>()
            .register_type::<TargetDirectory>()
            .register_type::<TargetModel>()
            .init_resource::<TargetArchive>()
            .init_resource::<TargetDirectory>()
            .init_resource::<TargetModel>()
            .add_systems(
                Last,
                (
                    pick_target_model,
                    mount_target_archive,
                    mount_target_directory,
                    load_target_model,
                )
                    .chain()
                    .after(ContentSet),
            );
    }
}

#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct TargetDirectory {
    pub current: Option<String>,
    #[reflect(ignore)]
    previous: Option<String>,
}

#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct TargetArchive {
    pub current: Option<String>,
    #[reflect(ignore)]
    previous: Option<String>,
}

#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct TargetModel {
    pub path: Option<String>,
    #[reflect(ignore)]
    pub model: Option<Entity>,
}

fn pick_target_model(
    mut archive: ResMut<TargetArchive>,
    mut directory: ResMut<TargetDirectory>,
    mut events: EventReader<DialogFilePicked<RenderBlockMesh>>,
    mut model: ResMut<TargetModel>,
) {
    for path in events.read().map(|e| e.path.clone()) {
        let Some(parent) = path.parent() else {
            continue;
        };
        let Ok(file) = path.strip_prefix(parent) else {
            continue;
        };

        archive.current = None;
        directory.current = Some(parent.to_string_lossy().to_string());
        model.path = Some(file.to_string_lossy().to_string());
    }
}

fn mount_target_archive(
    asset_server: Res<AssetServer>,
    mut archive: ResMut<TargetArchive>,
    mut mounts: ResMut<FileSystemMounts>,
) {
    if !archive.is_changed() || archive.current == archive.previous {
        return;
    }

    if let Some(previous_directory) = &archive.previous {
        mounts.unmount_archive(previous_directory.clone());
        archive.bypass_change_detection().previous = None;
    }

    if let Some(current_directory) = &archive.current {
        mounts.mount_archive(&asset_server, current_directory);
        archive.bypass_change_detection().previous = Some(current_directory.clone());
    }
}

fn mount_target_directory(
    mut directory: ResMut<TargetDirectory>,
    mut mounts: ResMut<FileSystemMounts>,
) {
    if !directory.is_changed() || directory.current == directory.previous {
        return;
    }

    if let Some(previous_directory) = &directory.previous {
        mounts.unmount_directory(previous_directory.clone());
        directory.bypass_change_detection().previous = None;
    }

    if let Some(current_directory) = &directory.current {
        mounts.mount_directory(current_directory);
        directory.bypass_change_detection().previous = Some(current_directory.clone());
    }
}

fn load_target_model(
    asset_server: Res<AssetServer>,
    mut target_model: ResMut<TargetModel>,
    mut commands: Commands,
) {
    if !target_model.is_changed() {
        return;
    }

    if let Some(model) = target_model.model {
        commands.entity(model).despawn_recursive();
    }

    if let Some(path) = &target_model.path {
        target_model.model = Some(
            commands
                .spawn(RenderBlockBundle {
                    mesh: asset_server.load(AssetPath::from(path)),
                    ..default()
                })
                .id(),
        );
    }
}

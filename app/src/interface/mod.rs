use std::path::PathBuf;

use bevy::{asset::AssetPath, prelude::*};
use bevy_egui::EguiSet;
use bevy_file_dialog::FileDialogPlugin;
use bevy_jc2_render_block::{RenderBlockBundle, RenderBlockMesh};

use crate::utilities::content::ContentDirectory;

use self::{
    file_tree::{draw_file_tree, process_file_tree, FileTreeEvent},
    title_bar::{draw_title_bar, open_model},
};

mod file_tree;
mod title_bar;

#[derive(Debug, Default)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            FileDialogPlugin::new()
                .with_pick_file::<RenderBlockMesh>()
                .with_save_file::<RenderBlockMesh>()
                .with_pick_directory::<ContentDirectory>(),
        )
        .add_event::<FileTreeEvent>()
        .init_resource::<TargetDirectory>()
        .init_resource::<TargetModel>()
        .add_systems(
            PreUpdate,
            (draw_title_bar, draw_file_tree, process_file_tree)
                .chain()
                .after(EguiSet::BeginFrame),
        )
        .add_systems(PostUpdate, (open_model, load_model));
    }
}

#[derive(Default, Debug, Resource)]
pub struct TargetDirectory {
    pub value: Option<PathBuf>,
}

#[derive(Default, Debug, Resource)]
pub struct TargetModel {
    pub path: Option<PathBuf>,
    pub model: Option<Entity>,
}

fn load_model(
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
                    mesh: asset_server.load(AssetPath::from_path(path)),
                    ..default()
                })
                .id(),
        );
    }
}

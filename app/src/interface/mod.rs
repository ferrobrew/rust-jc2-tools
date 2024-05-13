use bevy::prelude::*;
use bevy_egui::EguiSet;
use bevy_file_dialog::FileDialogPlugin;
use bevy_inspector_egui::quick::AssetInspectorPlugin;
use bevy_jc2_render_block::{materials::general::RenderBlockGeneralMaterial, RenderBlockMesh};

use crate::utilities::content::{ContentDirectory, ContentState};

use self::{
    file_tree::{draw_file_tree, process_file_tree, FileTreeEvent},
    title_bar::draw_title_bar,
};

mod file_tree;
mod title_bar;

#[derive(Debug, Default)]
pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FileDialogPlugin::new()
                .with_pick_file::<RenderBlockMesh>()
                .with_save_file::<RenderBlockMesh>()
                .with_pick_directory::<ContentDirectory>(),
            AssetInspectorPlugin::<RenderBlockGeneralMaterial>::new(),
        ))
        .add_event::<FileTreeEvent>()
        .add_systems(
            PreUpdate,
            (
                draw_title_bar,
                draw_file_tree.run_if(in_state(ContentState::Loaded)),
                process_file_tree,
            )
                .chain()
                .after(EguiSet::BeginFrame),
        );
    }
}

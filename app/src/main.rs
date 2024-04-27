use bevy::{asset::LoadState, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use render_block::{RenderBlockMesh, RenderBlockPlugin};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

use crate::render_block::materials::RenderBlockMaterial;

mod camera;
mod render_block;

#[derive(Bundle, Default)]
pub struct RenderBlockBundle {
    mesh: Handle<RenderBlockMesh>,
    transform: Transform,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RenderBlockPlugin,
            EguiPlugin,
            LookTransformPlugin,
            OrbitCameraPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, render_block_loader)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, camera::camera_input)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_rotate_sensitivity: Vec2::splat(0.8),
                mouse_translate_sensitivity: Vec2::splat(1.0),
                mouse_wheel_zoom_sensitivity: 0.1,
                ..default()
            },
            Vec3::new(7.5, 5.0, 7.5),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));

    let mesh = asset_server.load("sharkatron/go701_lod1-a.rbm");
    commands.spawn(RenderBlockBundle { mesh, ..default() });
}

fn render_block_loader(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    render_block_assets: Res<Assets<RenderBlockMesh>>,
    query: Query<(Entity, &Handle<RenderBlockMesh>)>,
) {
    for (entity, handle) in query.iter() {
        if asset_server.load_state(handle) == LoadState::Loaded {
            if let Some(mesh) = render_block_assets.get(handle) {
                for primitive in mesh.primitives.iter() {
                    commands.spawn(MaterialMeshBundle {
                        mesh: primitive.mesh.clone(),
                        material: match primitive.material.clone() {
                            RenderBlockMaterial::General(material) => material,
                        },
                        ..default()
                    });
                }
            }
            commands.entity(entity).remove::<Handle<RenderBlockMesh>>();
        }
    }
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

use bevy::{asset::LoadState, pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use render_block::{RenderBlockMesh, RenderBlockPlugin};

use crate::render_block::materials::RenderBlockMaterial;

mod render_block;

#[derive(Bundle, Default)]
pub struct RenderBlockBundle {
    mesh: Handle<RenderBlockMesh>,
    transform: Transform,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "JC2 Tools".into(),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
            RenderBlockPlugin,
            EguiPlugin,
            PanOrbitCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, render_block_loader)
        .add_systems(Update, ui_example_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(PanOrbitCamera {
            radius: Some(7.0),
            yaw: Some(45.0_f32.to_radians()),
            pitch: Some(45.0_f32.to_radians()),
            ..default()
        });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });

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

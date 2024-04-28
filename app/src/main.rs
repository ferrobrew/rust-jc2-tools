use bevy::{
    asset::LoadState,
    pbr::{
        wireframe::{Wireframe, WireframePlugin},
        CascadeShadowConfigBuilder,
    },
    prelude::*,
};
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
            WireframePlugin,
        ))
        .add_systems(Startup, startup_system)
        .add_systems(PreUpdate, render_block_system)
        .add_systems(Update, user_interface_system)
        .add_systems(Update, mesh_normals_system)
        .run();
}

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            45_f32.to_radians(),
            45_f32.to_radians(),
            0_f32.to_radians(),
        )),
        ..default()
    });

    let mesh = asset_server.load("traincar01/gp040_lod1-e.rbm");
    commands.spawn(RenderBlockBundle { mesh, ..default() });
}

fn render_block_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    render_block_assets: Res<Assets<RenderBlockMesh>>,
    query: Query<(Entity, &Handle<RenderBlockMesh>)>,
) {
    for (entity, handle) in query.iter() {
        if asset_server.load_state(handle) == LoadState::Loaded {
            if let Some(mesh) = render_block_assets.get(handle) {
                for primitive in mesh.primitives.iter() {
                    commands
                        .spawn(MaterialMeshBundle {
                            mesh: primitive.mesh.clone(),
                            material: match primitive.material.clone() {
                                RenderBlockMaterial::General(material) => material,
                            },
                            transform: Transform::from_scale(Vec3::splat(5.0)),
                            ..default()
                        })
                        .insert(Wireframe);
                }
            }
            commands.entity(entity).remove::<Handle<RenderBlockMesh>>();
        }
    }
}

fn mesh_normals_system(
    mut gizmos: Gizmos,
    meshes: Res<Assets<Mesh>>,
    query: Query<(&Transform, &Handle<Mesh>, &Wireframe)>,
) {
    for (transform, mesh, _) in &query {
        if let Some(mesh) = meshes.get(mesh) {
            let positions = mesh
                .attribute(Mesh::ATTRIBUTE_POSITION)
                .and_then(|x| x.as_float3());
            let normals = mesh
                .attribute(Mesh::ATTRIBUTE_NORMAL)
                .and_then(|x| x.as_float3());
            let tangents = mesh
                .attribute(Mesh::ATTRIBUTE_TANGENT)
                .and_then(|x| match x {
                    bevy::render::mesh::VertexAttributeValues::Float32x4(values) => Some(values),
                    _ => None,
                });

            if let (Some(positions), Some(normals), Some(tangents)) = (positions, normals, tangents)
            {
                for i in 0..mesh.count_vertices() {
                    let (position, normal, tangent): (Vec3, Vec3, Vec4) =
                        (positions[i].into(), normals[i].into(), tangents[i].into());

                    let p = transform.transform_point(position);
                    let n = transform.rotation * normal;
                    let t = transform.rotation * tangent.xyz();
                    let b = n.cross(t) * tangent[3];

                    gizmos.arrow(p, p + n.normalize() * 0.05, Color::RED);
                    gizmos.arrow(p, p + t.normalize() * 0.05, Color::GREEN);
                    gizmos.arrow(p, p + b.normalize() * 0.05, Color::BLUE);
                }
            }
        }
    }
}

fn user_interface_system(
    mut contexts: EguiContexts,
    mut query: Query<(&mut Transform, &DirectionalLight)>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui.label("Directional Light");
        for (mut transform, _light) in &mut query {
            let (mut x, mut y, mut z) = transform.rotation.to_euler(EulerRot::XYZ);
            ui.drag_angle(&mut x);
            ui.drag_angle(&mut y);
            ui.drag_angle(&mut z);
            transform.rotation = Quat::from_euler(EulerRot::XYZ, x, y, z);
        }
    });
}

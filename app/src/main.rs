use bevy::{
    pbr::{
        wireframe::{WireframeConfig, WireframePlugin},
        CascadeShadowConfigBuilder,
    },
    prelude::*,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use debug::wireframe::{WireframeNormalsConfig, WireframeNormalsPlugin};
use render_block::{RenderBlockBundle, RenderBlockPlugin};

mod debug;
mod render_block;

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
            WireframeNormalsPlugin,
        ))
        .insert_resource(WireframeConfig {
            default_color: Color::WHITE,
            ..default()
        })
        .add_systems(Startup, startup_system)
        .add_systems(Update, user_interface_system)
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

    let mesh = asset_server.load("traincar01/gp040_lod1-a.rbm");
    commands.spawn(RenderBlockBundle { mesh, ..default() });
}

fn user_interface_system(
    mut contexts: EguiContexts,
    mut wireframes: ResMut<WireframeConfig>,
    mut normals: ResMut<WireframeNormalsConfig>,
    mut query: Query<(&mut Transform, &DirectionalLight)>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        ui.checkbox(&mut wireframes.global, "Wireframes");
        ui.checkbox(&mut normals.global, "Normals");
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

use bevy::{
    pbr::{
        wireframe::{WireframeConfig, WireframePlugin},
        CascadeShadowConfigBuilder,
    },
    prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_jc2_file_system::FileSystemPlugin;
use bevy_jc2_render_block::RenderBlockPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use debug::wireframe::WireframeNormalsPlugin;
use interface::InterfacePlugin;
use utilities::{content::ContentPlugin, streaming::StreamingPlugin};

mod debug;
mod interface;
mod utilities;

fn main() {
    App::new()
        .add_plugins((
            FileSystemPlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "JC2 Tools".into(),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
            ContentPlugin,
            StreamingPlugin,
            InterfacePlugin,
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
        .run();
}

fn startup_system(mut commands: Commands) {
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
}

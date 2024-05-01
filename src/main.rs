use bevy::{
    pbr::{
        wireframe::{WireframeConfig, WireframePlugin},
        CascadeShadowConfigBuilder,
    },
    prelude::*,
    winit::WinitWindows,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_file_dialog::{FileDialogExt, FileDialogPlugin};
use bevy_jc2_render_block::{RenderBlockBundle, RenderBlockMesh, RenderBlockPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use debug::wireframe::{WireframeNormalsConfig, WireframeNormalsPlugin};

mod debug;

#[derive(Resource, Debug, Clone, Default, Reflect)]
#[reflect(Resource)]
pub struct AppData {
    pub file: String,
    pub model: Option<Entity>,
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
            FileDialogPlugin::new()
                .with_pick_file::<RenderBlockMesh>()
                .with_save_file::<RenderBlockMesh>(),
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
        .insert_resource(AppData {
            file: "landmarks/Landmark_LOD1-KEY001.rbm".into(),
            ..default()
        })
        .add_systems(Startup, startup_system)
        .add_systems(Update, user_interface_system)
        .run();
}

fn startup_system(
    mut app_data: ResMut<AppData>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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

    app_data.model = Some(
        commands
            .spawn(RenderBlockBundle {
                mesh: asset_server.load(&app_data.file),
                ..default()
            })
            .id(),
    );
}

fn user_interface_system(
    mut app_data: ResMut<AppData>,
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut normals: ResMut<WireframeNormalsConfig>,
    mut query: Query<(&mut Transform, &DirectionalLight)>,
    mut wireframes: ResMut<WireframeConfig>,
    _windows: NonSend<WinitWindows>,
) {
    egui::TopBottomPanel::top("title_bar").show(contexts.ctx_mut(), |ui| {
        ui.visuals_mut().button_frame = false;
        ui.horizontal_wrapped(|ui| {
            ui.menu_button("File", |ui| {
                ui.menu_button("Open", |ui| {
                    ui.text_edit_singleline(&mut app_data.file);
                    if ui.button("Load File").clicked() {
                        commands
                            .dialog()
                            .add_filter("Render Block Model", &["rbm"])
                            .pick_file_path::<RenderBlockMesh>();
                    }
                });
                let _ = ui.button("Save");
                let _ = ui.button("Close");
            });

            ui.separator();
            ui.menu_button("View", |ui| {
                ui.checkbox(&mut wireframes.global, "Wireframes");

                ui.checkbox(&mut normals.global, "Normals");
                if normals.global {
                    ui.add(
                        egui::Slider::new(&mut normals.length, 0.01..=0.1)
                            .text("Normals Length")
                            .custom_formatter(|n, _| format!("{n:.3}")),
                    );
                }

                ui.label("Directional Light");
                for (mut transform, _light) in &mut query {
                    let (mut x, mut y, mut z) = transform.rotation.to_euler(EulerRot::XYZ);
                    ui.horizontal(|ui| {
                        ui.drag_angle(&mut x);
                        ui.drag_angle(&mut y);
                        ui.drag_angle(&mut z);
                    });
                    transform.rotation = Quat::from_euler(EulerRot::XYZ, x, y, z);
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });
    });
}

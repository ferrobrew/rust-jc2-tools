use std::{fs, path::PathBuf};

use bevy::{
    pbr::{
        wireframe::{WireframeConfig, WireframePlugin},
        CascadeShadowConfigBuilder,
    },
    prelude::*,
    winit::WinitWindows,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_file_dialog::{DialogDirectoryPicked, DialogFilePicked, FileDialogExt, FileDialogPlugin};
use bevy_jc2_file_system::{FileSystemMounts, FileSystemPlugin};
use bevy_jc2_render_block::{RenderBlockBundle, RenderBlockMesh, RenderBlockPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use debug::wireframe::{WireframeNormalsConfig, WireframeNormalsPlugin};
use itertools::Itertools;

mod debug;

struct ContentDirectory;

#[derive(Resource, Debug, Clone, Default, Reflect)]
#[reflect(Resource)]
pub struct AppData {
    pub file: String,
    pub file_directory: Option<PathBuf>,
    pub content_archives: Vec<PathBuf>,
    pub content_directory: Option<PathBuf>,
    pub model: Option<Entity>,
}

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
            FileDialogPlugin::new()
                .with_pick_file::<RenderBlockMesh>()
                .with_save_file::<RenderBlockMesh>()
                .with_pick_directory::<ContentDirectory>(),
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
        .add_systems(PostUpdate, (open_render_block, open_content_directory))
        .run();
}

fn startup_system(
    asset_server: Res<AssetServer>,
    mut app_data: ResMut<AppData>,
    mut commands: Commands,
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

fn open_render_block(
    asset_server: Res<AssetServer>,
    mounts: Res<FileSystemMounts>,
    mut app_data: ResMut<AppData>,
    mut commands: Commands,
    mut events: EventReader<DialogFilePicked<RenderBlockMesh>>,
) {
    for path in events.read().map(|e| e.path.clone()) {
        let Some(directory) = path.parent() else {
            continue;
        };
        let Ok(file) = path.strip_prefix(directory) else {
            continue;
        };

        // Remount file directory
        if let Some(mounted_directory) = app_data.file_directory.clone() {
            mounts.unmount_directory(mounted_directory);
        }
        mounts.mount_directory(directory);
        app_data.file_directory = Some(directory.into());

        // Despawn existing model
        if let Some(model) = app_data.model {
            commands.entity(model).despawn_recursive();
        }

        // Spawn the new model
        app_data.file = file.to_string_lossy().into();
        app_data.model = Some(
            commands
                .spawn(RenderBlockBundle {
                    mesh: asset_server.load(&app_data.file),
                    ..default()
                })
                .id(),
        );
    }
}

fn open_content_directory(
    asset_server: Res<AssetServer>,
    mut mounts: ResMut<FileSystemMounts>,
    mut app_data: ResMut<AppData>,
    mut events: EventReader<DialogDirectoryPicked<ContentDirectory>>,
) {
    for path in events.read().map(|e| e.path.clone()) {
        // Unmount and clear content archives
        for archive in &app_data.content_archives {
            mounts.unmount_archive(archive);
        }
        app_data.content_archives.clear();

        // Remount the content directory
        if let Some(content_directory) = &app_data.content_directory {
            mounts.unmount_directory(content_directory);
        }
        mounts.mount_directory(path.clone());
        app_data.content_directory = Some(path.clone());

        // Discover archives
        let archives = ["archives_win32", "DLC"]
            .iter()
            .filter_map(|directory| fs::read_dir(path.join(directory)).ok())
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
            .filter_map(|archive| archive.strip_prefix(&path).ok())
        {
            mounts.mount_archive(&asset_server, archive.to_owned());
            app_data.content_archives.push(archive.into());
        }
    }
}

fn user_interface_system(
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
                if ui.button("Open").clicked() {
                    commands
                        .dialog()
                        .add_filter("Render Block Model", &["rbm"])
                        .pick_file_path::<RenderBlockMesh>();
                }
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

            ui.separator();
            ui.menu_button("Options", |ui| {
                if ui.button("Mount Content").clicked() {
                    commands
                        .dialog()
                        .add_filter("Render Block Model", &["rbm"])
                        .pick_directory_path::<ContentDirectory>();
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });
    });
}

use bevy::{pbr::wireframe::WireframeConfig, prelude::*, winit::WinitWindows};
use bevy_egui::{EguiContexts, egui};
use bevy_file_dialog::FileDialogExt;
use bevy_jc2_render_block::RenderBlockMesh;

use crate::{debug::wireframe::WireframeNormalsConfig, utilities::content::ContentDirectory};

pub fn draw_title_bar(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut normals: ResMut<WireframeNormalsConfig>,
    mut query: Query<(&mut Transform, &DirectionalLight)>,
    mut wireframes: ResMut<WireframeConfig>,
    _windows: NonSend<WinitWindows>,
) {
    egui::TopBottomPanel::top("title_bar").show(contexts.ctx_mut(), |ui| {
        ui.visuals_mut().button_frame = false;
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    ui.close_menu();
                    commands
                        .dialog()
                        .add_filter("Render Block Model", &["rbm"])
                        .pick_file_path::<RenderBlockMesh>();
                }
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
                    ui.close_menu();
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

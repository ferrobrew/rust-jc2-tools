use std::path::PathBuf;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use bevy_jc2_file_system::{FileSystemMounts, FileSystemTreeIterValue};

use crate::utilities::model::{TargetArchive, TargetDirectory, TargetModel};

#[derive(Event, Debug)]
pub enum FileTreeEvent {
    MountArchive { path: PathBuf },
    UnmountArchive { path: PathBuf },
    LoadModel { path: PathBuf },
}

pub fn draw_file_tree(
    mounts: Res<FileSystemMounts>,
    mut contexts: EguiContexts,
    mut event_writer: EventWriter<FileTreeEvent>,
) {
    egui::SidePanel::left("file_tree")
        .default_width(300.0)
        .max_width(500.0)
        .show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                draw(&mut event_writer, &mounts, &mounts.file_tree, ui);
            });
        });

    fn draw<'a>(
        event_writer: &mut EventWriter<FileTreeEvent>,
        mounts: &FileSystemMounts,
        tree: impl IntoIterator<Item = FileSystemTreeIterValue<'a>>,
        ui: &mut egui::Ui,
    ) {
        for entry in tree {
            ui.horizontal(|ui| {
                let response = if !entry.is_empty() {
                    egui::CollapsingHeader::new(entry.name())
                        .show(ui, |ui| {
                            draw(event_writer, mounts, &entry, ui);
                        })
                        .header_response
                } else {
                    ui.add(
                        egui::Label::new(entry.name())
                            .selectable(false)
                            .sense(egui::Sense::click()),
                    )
                };
                let context_menu = |menu: &mut dyn FnMut(&mut egui::Ui)| {
                    response
                        .interact(egui::Sense::hover())
                        .context_menu(|ui| menu(ui));
                };

                let path = entry.path();
                let Some(extension) = path.extension().and_then(|e| e.to_str()) else {
                    return;
                };

                match extension {
                    "bl" | "blz" | "ee" | "eez" | "fl" | "flz" | "nl" | "nlz" => {
                        context_menu(&mut |ui| {
                            let mounted = mounts.has_mounted_archive(path);
                            let label = if mounted { "unmount" } else { "mount" };

                            ui.add_enabled_ui(!mounts.is_mounting_archive(path), |ui| {
                                if ui.button(label).clicked() {
                                    event_writer.send(if mounted {
                                        FileTreeEvent::UnmountArchive { path: path.into() }
                                    } else {
                                        FileTreeEvent::MountArchive { path: path.into() }
                                    });
                                    ui.close_menu();
                                }
                            });
                        });
                    }
                    "rbm" => {
                        context_menu(&mut |ui| {
                            ui.add_enabled_ui(!mounts.is_mounting_archive(path), |ui| {
                                if ui.button("load").clicked() {
                                    event_writer
                                        .send(FileTreeEvent::LoadModel { path: path.into() });
                                    ui.close_menu();
                                }
                            });
                        });
                    }
                    _ => {}
                };
            });
        }
    }
}

pub fn process_file_tree(
    asset_server: Res<AssetServer>,
    mut archive: ResMut<TargetArchive>,
    mut directory: ResMut<TargetDirectory>,
    mut events: EventReader<FileTreeEvent>,
    mut model: ResMut<TargetModel>,
    mut mounts: ResMut<FileSystemMounts>,
) {
    for event in events.read() {
        match event {
            FileTreeEvent::MountArchive { path } => {
                mounts.mount_archive(&asset_server, path);
            }
            FileTreeEvent::UnmountArchive { path } => {
                mounts.unmount_archive(path);
            }
            FileTreeEvent::LoadModel { path } => {
                let Some(archive_path) = path.parent() else {
                    continue;
                };
                directory.current = None;
                archive.current = Some(archive_path.to_string_lossy().to_string());
                model.path = Some(path.to_string_lossy().to_string());
            }
        }
    }
}

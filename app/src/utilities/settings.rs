use bevy::{pbr::wireframe::WireframeConfig, prelude::*};
use bevy_save::prelude::*;

use crate::{debug::wireframe::WireframeNormalsConfig, utilities::content::ContentDirectory};

use super::model::{TargetArchive, TargetDirectory, TargetModel};

#[derive(Debug, Default)]
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SavePlugin)
            .add_systems(PostStartup, load)
            .add_systems(
                Last,
                save.run_if(
                    resource_changed::<ContentDirectory>
                        .and_then(not(resource_added::<ContentDirectory>))
                        .or_else(
                            resource_changed::<TargetArchive>
                                .and_then(not(resource_added::<TargetArchive>)),
                        )
                        .or_else(
                            resource_changed::<TargetDirectory>
                                .and_then(not(resource_added::<TargetDirectory>)),
                        )
                        .or_else(
                            resource_changed::<TargetModel>
                                .and_then(not(resource_added::<TargetModel>)),
                        )
                        .or_else(
                            resource_changed::<WireframeConfig>
                                .and_then(not(resource_added::<WireframeConfig>)),
                        )
                        .or_else(
                            resource_changed::<WireframeNormalsConfig>
                                .and_then(not(resource_added::<WireframeNormalsConfig>)),
                        ),
                ),
            );
    }
}

struct SettingsPipeline;

impl Pipeline for SettingsPipeline {
    type Backend = DefaultDebugBackend;
    type Format = DefaultDebugFormat;
    type Key<'a> = &'a str;

    fn key(&self) -> Self::Key<'_> {
        "settings"
    }

    fn capture(builder: SnapshotBuilder) -> Snapshot {
        builder
            .extract_resource::<ContentDirectory>()
            .extract_resource::<TargetArchive>()
            .extract_resource::<TargetDirectory>()
            .extract_resource::<TargetModel>()
            .extract_resource::<WireframeConfig>()
            .extract_resource::<WireframeNormalsConfig>()
            .build()
    }
}

fn load(world: &mut World) {
    if world.load(SettingsPipeline).is_err() {
        bevy::log::warn!("failed to load settings");
    }
}

fn save(world: &mut World) {
    if world.save(SettingsPipeline).is_err() {
        bevy::log::warn!("failed to save settings");
    }
}

#![allow(unsafe_code)]

use godot::prelude::*;

mod property_container;
mod resource_loader;

struct Extension;

#[gdextension]
/// SAFETY: Presumably unsafe due to `gdext_rust_init` symbol export. Ensure this symbol is unique.
unsafe impl ExtensionLibrary for Extension {
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::Scene {
            resource_loader::JcResourceLoader::register();
        }
    }

    fn on_stage_deinit(stage: InitStage) {
        if stage == InitStage::Scene {
            resource_loader::JcResourceLoader::unregister();
        }
    }
}

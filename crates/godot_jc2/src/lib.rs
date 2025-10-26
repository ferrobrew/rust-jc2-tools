#![allow(unsafe_code)]

use godot::prelude::*;

mod resource_loader;

struct Extension;

#[gdextension]
/// SAFETY: Presumably unsafe due to `gdext_rust_init` symbol export. Ensure this symbol is unique.
unsafe impl ExtensionLibrary for Extension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            resource_loader::JcResourceLoader::register();
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            resource_loader::JcResourceLoader::unregister();
        }
    }
}

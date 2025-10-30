use std::collections::HashMap;

use godot::prelude::*;

use super::*;

mod lod;
pub use lod::JcLod;

mod model;
pub use model::JcModel;

mod terrain;
pub use terrain::JcTerrain;

mod texture;
pub use texture::JcTexture;

pub type JcResourceFormats = HashMap<GString, JcResourceFormatImpl>;
pub type JcResourceFormatImpl =
    fn(GString, PackedByteArray, &mut JcResourceThread) -> JcResourceResult<Gd<Object>>;

trait JcResourceFormat {
    const EXTENSION: &str;
    type Result: GodotClass + Inherits<Object>;

    fn register() -> (GString, JcResourceFormatImpl) {
        (GString::from(Self::EXTENSION), Self::load_internal)
    }

    #[inline]
    #[doc(hidden)]
    fn load_internal(
        path: GString,
        buffer: PackedByteArray,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Object>> {
        Ok(Self::from_buffer(path, buffer, thread)?.upcast())
    }

    #[inline]
    fn from_path(
        path: GString,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>> {
        let buffer = thread.get_buffer(&path)?;
        Self::from_buffer(path, buffer, thread)
    }

    fn from_buffer(
        path: GString,
        buffer: PackedByteArray,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Self::Result>>;
}

pub fn register() -> JcResourceFormats {
    HashMap::from([
        JcLod::register(),
        JcModel::register(),
        JcTerrain::register(),
        JcTexture::register(),
    ])
}

use std::collections::HashMap;

use godot::prelude::*;

use super::*;

mod lod;
pub use lod::JcLod;

mod model_collection;
pub use model_collection::JcModelCollection;

mod model;
pub use model::JcModel;

mod properties;
pub use properties::JcProperties;

mod terrain;
pub use terrain::JcTerrain;

mod texture;
pub use texture::JcTexture;

pub type JcResourceFormats = HashMap<GString, JcResourceFormatImpl>;
pub type JcResourceFormatImpl =
    fn(GString, PackedByteArray, &mut JcResourceThread) -> JcResourceResult<Gd<Object>>;

trait JcResourceFormat<const EXTENSIONS_COUNT: usize = 1> {
    const EXTENSIONS: [&str; EXTENSIONS_COUNT];
    type Result: GodotClass + Inherits<Object>;

    fn register(formats: &mut JcResourceFormats) {
        for extension in Self::EXTENSIONS {
            formats.insert(extension.into(), Self::load_internal);
        }
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
    let mut formats = JcResourceFormats::new();
    JcLod::register(&mut formats);
    JcModelCollection::register(&mut formats);
    JcModel::register(&mut formats);
    JcProperties::register(&mut formats);
    JcTerrain::register(&mut formats);
    JcTexture::register(&mut formats);
    formats
}

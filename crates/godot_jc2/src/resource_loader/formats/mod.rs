use std::collections::HashMap;

use godot::prelude::*;

use super::*;

mod lod;
mod model;
mod terrain;
mod texture;

pub type JcResourceFormats = HashMap<GString, JCResourceFormatImpl>;
pub type JcResourceFormat = (GString, JCResourceFormatImpl);
pub type JCResourceFormatImpl =
    fn(GString, PackedByteArray, &mut JcResourceThread) -> JcResourceResult<Gd<Object>>;

pub fn register() -> JcResourceFormats {
    HashMap::from([
        lod::register(),
        model::register(),
        terrain::register(),
        texture::register(),
    ])
}

use binrw::binrw;

use crate::string::LengthString;

#[binrw]
#[brw(repr = u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum PrimitiveType {
    #[default]
    TriangleList,
    TriangleStrip,
    TriangleFan,
    IndexedTriangleList,
    IndexedTriangleStrip,
    IndexedTriangleFan,
    LineList,
    PointSprite,
    IndexedPointSprite,
}

#[binrw]
#[derive(Clone, Debug, Default)]
pub struct Material {
    pub textures: [LengthString<u32>; Material::MAX_TEXTURE_COUNT],
    pub primitive_type: PrimitiveType,
}

impl Material {
    pub const MAX_TEXTURE_COUNT: usize = 8;
}

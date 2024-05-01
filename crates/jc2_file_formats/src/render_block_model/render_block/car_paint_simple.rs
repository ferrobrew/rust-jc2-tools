use binrw::binrw;

use crate::render_block_model::{IndexBuffer, Material, SimpleVertex, VertexBuffer};

use super::CarPaintAttributes;

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum CarPaintSimpleVersion {
    #[default]
    V1 = 1,
}

#[binrw]
#[derive(Clone, Debug, Default)]
pub struct CarPaintSimpleRenderBlock {
    pub version: CarPaintSimpleVersion,
    pub attributes: CarPaintAttributes,
    pub material: Material,
    pub vertices: VertexBuffer<SimpleVertex>,
    #[brw(args(vertices.len()))]
    pub indices: IndexBuffer<u16>,
}

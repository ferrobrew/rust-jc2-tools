use binrw::binrw;

use crate::render_block_model::{HaloVertex, IndexBuffer, Material, VertexBuffer};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum HaloVersion {
    #[default]
    V0,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct HaloRenderBlock {
    pub version: HaloVersion,
    pub material: Material,
    pub vertices: VertexBuffer<HaloVertex>,
    #[brw(args(vertices.len()))]
    pub indices: IndexBuffer<u16>,
}

use binrw::binrw;

use crate::render_block_model::{BillboardFoliageVertex, IndexBuffer, Material, VertexBuffer};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum BillboardFoliageVersion {
    #[default]
    V0,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct BillboardFoliageRenderBlock {
    pub version: BillboardFoliageVersion,
    pub material: Material,
    pub vertices: VertexBuffer<BillboardFoliageVertex>,
    #[brw(args(vertices.len()))]
    pub indices: IndexBuffer<u16>,
}

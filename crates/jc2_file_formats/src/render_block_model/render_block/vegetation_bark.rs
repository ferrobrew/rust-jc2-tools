use binrw::binrw;
use bitflags::bitflags;

use crate::render_block_model::{IndexBuffer, Material, VegetationVertex, VertexBuffer};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum VegetationBarkVersion {
    #[default]
    V0,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
    pub struct VegetationBarkFlags: u32 {
        const USE_LOW_RES_SHADOWS = 1 << 0;
        const USE_WATER_FOG = 1 << 1;
        const NO_DIRT_MAP = 1 << 2;
        const NO_FADE = 1 << 3;
    }
}

#[binrw]
#[derive(Clone, Debug, Default)]
pub struct VegetationBarkAttributes {
    pub flags: VegetationBarkFlags,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct VegetationBarkRenderBlock {
    pub version: VegetationBarkVersion,
    pub attributes: VegetationBarkAttributes,
    pub material: Material,
    #[brw(args(attributes.flags.intersects(VegetationBarkFlags::NO_DIRT_MAP)))]
    pub vertices: VertexBuffer<VegetationVertex>,
    #[brw(args(vertices.len()))]
    pub indices: IndexBuffer<u16>,
}

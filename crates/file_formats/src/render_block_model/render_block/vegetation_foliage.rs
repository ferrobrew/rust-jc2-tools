use binrw::binrw;
use bitflags::bitflags;

use crate::{
    math::Vec3,
    render_block_model::{IndexBuffer, Material, VegetationVertex, VertexBuffer},
};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum VegetationFoliageVersion {
    #[default]
    V0,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
    pub struct VegetationFoliageFlags: u32 {
        const NO_SPECULAR = 1 << 0;
        const NO_TRANSLUCENCY = 1 << 1;
        const NO_SNOW = 1 << 2;
        const USE_WATER_FOG = 1 << 3;
        const USE_LOW_RES_SHADOWS = 1 << 4;
        const NO_ALPHA_TEST = 1 << 5;
        const USE_SIMPLE_SHADER = 1 << 6;
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct VegetationFoliageAttributes {
    pub specular_intensity: f32,
    pub specular_power: f32,
    pub translucency_mask: Vec3<f32>,
    pub flags: VegetationFoliageFlags,
}

impl Default for VegetationFoliageAttributes {
    fn default() -> Self {
        Self {
            specular_intensity: 0.5,
            specular_power: 10.0,
            translucency_mask: Vec3::new(1.6, 1.8, 1.0),
            flags: Default::default(),
        }
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct VegetationFoliageRenderBlock {
    pub version: VegetationFoliageVersion,
    pub attributes: VegetationFoliageAttributes,
    pub material: Material,
    #[brw(args(false))]
    pub vertices: VertexBuffer<VegetationVertex>,
    #[brw(args(vertices.len()))]
    pub indices: IndexBuffer<u16>,
}

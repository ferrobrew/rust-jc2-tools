use binrw::binrw;
use bitflags::bitflags;

use crate::{
    math::{Vec3, Vec4},
    render_block_model::{FacadeVertex, IndexBuffer, Material, VertexBuffer, VertexFormat},
};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum FacadeVersion {
    #[default]
    V1 = 1,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
    pub struct FacadeFlags: u32 {
        const NO_CULLING = 1 << 0;
        const ALPHA_BLENDING = 1 << 1;
        const USE_CHANNEL_DIRT = 1 << 4;
        const USE_CHANNEL_TEXTURES = 1 << 5;
        const USE_SNOW_FLAG = 1 << 6;
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct FacadeAttributes {
    pub channel_mask: Vec4<f32>,
    pub channel_dirt_mask: Vec3<f32>,
    pub emissive_multiplier: Vec3<f32>,
    pub depth_bias: f32,
    pub specular_power: f32,
    pub vertex_format: VertexFormat,
    pub scale: f32,
    pub flags: FacadeFlags,
}

impl Default for FacadeAttributes {
    #[inline]
    fn default() -> Self {
        Self {
            channel_mask: Vec4::splat(1.0),
            channel_dirt_mask: Vec3::splat(1.0),
            emissive_multiplier: Vec3::new(1.8, 1.8, 1.6),
            depth_bias: 0.0,
            specular_power: 16.0,
            vertex_format: Default::default(),
            scale: 1.0,
            flags: Default::default(),
        }
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct FacadeRenderBlock {
    pub version: FacadeVersion,
    pub attributes: FacadeAttributes,
    pub material: Material,
    #[brw(args(attributes.vertex_format))]
    pub vertices: VertexBuffer<FacadeVertex>,
    #[brw(args(vertices.len()))]
    pub indices: IndexBuffer<u16>,
}

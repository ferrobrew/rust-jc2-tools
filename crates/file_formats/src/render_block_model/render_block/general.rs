use binrw::binrw;
use bitflags::bitflags;

use crate::{
    math::Vec4,
    render_block_model::{GeneralVertex, IndexBuffer, Material, VertexBuffer, VertexInfo},
};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum GeneralVersion {
    V1 = 1,
    V2,
    #[default]
    V3,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
    pub struct GeneralFlags: u32 {
        const NO_CULLING = 1 << 0;
        const ALPHA_BLENDING = 1 << 1;
        const ADDITIVE_ALPHA = 1 << 2;
        const USE_PALETTE = 1 << 3;
        const USE_SUB_SURFACE_SCATTERING = 1 << 4;
        const USE_CHANNEL_TEXTURES = 1 << 5;
        const USE_SNOW_FLAG = 1 << 6;
        const ANIMATE_TEXTURE = 1 << 7;
        const ALPHA_TEST = 1 << 8;
        const USE_AMBIENT_OCCLUSION = 1 << 9;
        const NO_DEPTH_TEST = 1 << 10;
    }
}

#[binrw]
#[brw(import(
    version: &GeneralVersion
))]
#[derive(Clone, Debug)]
pub struct GeneralAttributes {
    pub channel_mask: Vec4<f32>,
    pub channel_ambient_occlusion_mask: Vec4<f32>,
    pub depth_bias: f32,
    pub specular_power: f32,
    #[brw(args(matches!(version, GeneralVersion::V3)))]
    pub vertex_info: VertexInfo,
    pub flags: GeneralFlags,
}

impl Default for GeneralAttributes {
    #[inline]
    fn default() -> Self {
        Self {
            channel_mask: Vec4::splat(0.0),
            channel_ambient_occlusion_mask: Vec4::splat(0.0),
            depth_bias: 0.0,
            specular_power: 16.0,
            vertex_info: Default::default(),
            flags: GeneralFlags::USE_SNOW_FLAG,
        }
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct GeneralRenderBlock {
    pub version: GeneralVersion,
    #[brw(args(&version))]
    pub attributes: GeneralAttributes,
    pub material: Material,
    #[brw(args(attributes.vertex_info.format))]
    pub vertices: VertexBuffer<GeneralVertex>,
    #[brw(args(vertices.len()))]
    pub indices: IndexBuffer<u16>,
}

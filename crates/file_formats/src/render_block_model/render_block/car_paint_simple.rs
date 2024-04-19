use binrw::binrw;
use bitflags::bitflags;

use crate::{
    math::{Vec3, Vec4},
    render_block_model::{Indices, SimpleVertex, Vertices},
};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum CarPaintSimpleVersion {
    #[default]
    V1 = 1,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
    pub struct CarPaintFlags: u32 {
        const NO_CULLING = 1 << 0;
        const ALPHA_BLENDING = 1 << 1;
        const IGNORE_PALETTE = 1 << 3;
        const NO_DIRT = 1 << 4;
        const DECAL = 1 << 5;
        const MASK_WATER = 1 << 6;
        const ALPHA_TEST = 1 << 7;
        const DULL = 1 << 8;
        const IS_LIGHT = 1 << 9;
        const FLAT_NORMAL = 1 << 10;
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct CarPaintAttributes {
    pub two_tone_colors: [Vec3<f32>; 2],
    pub specular_power: f32,
    pub depth_bias: f32,
    pub reflection_multiplier: f32,
    pub noise_factors: Vec4<f32>,
    pub flags: CarPaintFlags,
}

impl Default for CarPaintAttributes {
    #[inline]
    fn default() -> Self {
        Self {
            two_tone_colors: [
                Vec3::new(0.784, 0.784, 0.784),
                Vec3::new(0.651, 0.729, 0.827),
            ],
            specular_power: 64.0,
            depth_bias: 0.0,
            reflection_multiplier: 0.5,
            noise_factors: Vec4::splat(0.0),
            flags: Default::default(),
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Default)]
pub struct CarPaintSimpleRenderBlock {
    pub version: CarPaintSimpleVersion,
    pub attributes: CarPaintAttributes,
    pub vertices: Vertices<SimpleVertex>,
    #[brw(args(vertices.len()))]
    pub indices: Indices<u16>,
}

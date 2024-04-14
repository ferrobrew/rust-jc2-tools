use binrw::binrw;

use crate::math::{Vec2, Vec4};

#[binrw]
#[brw(repr = u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum VertexFormat {
    #[default]
    F32,
    I16,
}

#[binrw]
#[brw(import(
    has_uv1: bool
))]
#[derive(Clone, Debug)]
pub struct VertexInfo {
    pub format: VertexFormat,
    pub scale: f32,
    pub uv0_extent: Vec2<f32>,
    #[brw(if(has_uv1))]
    pub uv1_extent: Vec2<f32>,
    pub color_extent: f32,
    pub color: Vec4<u8>,
}

impl Default for VertexInfo {
    fn default() -> Self {
        Self {
            format: Default::default(),
            scale: 1.0,
            uv0_extent: Vec2::splat(1.0),
            uv1_extent: Vec2::splat(1.0),
            color_extent: 1.0,
            color: Vec4::splat(255),
        }
    }
}

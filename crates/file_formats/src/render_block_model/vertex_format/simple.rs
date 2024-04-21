use binrw::binrw;

use crate::{
    math::{Vec2, Vec3},
    render_block_model::PackedNormalF32,
};

use super::{GenericVertex, Vertex};

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SimpleVertex {
    pub position: Vec3<f32>,
    #[br(map = PackedNormalF32::into)]
    #[bw(map = |x| PackedNormalF32::from(*x))]
    pub normal: Vec3<f32>,
    pub uv0: Vec2<f32>,
    #[br(map = PackedNormalF32::into)]
    #[bw(map = |x| PackedNormalF32::from(*x))]
    pub tangent: Vec3<f32>,
    #[br(map = PackedNormalF32::into)]
    #[bw(map = |x| PackedNormalF32::from(*x))]
    pub binormal: Vec3<f32>,
}

impl Vertex for SimpleVertex {
    type VertexArgs = ();
}

impl From<GenericVertex> for SimpleVertex {
    #[inline]
    fn from(value: GenericVertex) -> Self {
        value.into()
    }
}

impl From<SimpleVertex> for GenericVertex {
    #[inline]
    fn from(value: SimpleVertex) -> Self {
        Self {
            position: value.position,
            normal: value.normal,
            uv0: value.uv0,
            tangent: value.tangent,
            binormal: value.binormal,
            ..Default::default()
        }
    }
}

use binrw::binrw;

use crate::{
    math::{Vec2, Vec3},
    render_block_model::PackedNormal,
};

use super::{GenericVertex, Vertex};

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SimpleVertex {
    pub position: Vec3<f32>,
    pub normal: PackedNormal,
    pub uv0: Vec2<f32>,
    pub tangent: PackedNormal,
    pub binormal: PackedNormal,
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
            normal: value.normal.into(),
            uv0: value.uv0,
            tangent: value.tangent.into(),
            binormal: value.binormal.into(),
            ..Default::default()
        }
    }
}

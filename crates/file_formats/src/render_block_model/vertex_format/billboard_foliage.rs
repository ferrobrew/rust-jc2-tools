use binrw::binrw;

use crate::math::{Vec2, Vec3};

use super::{GenericVertex, Vertex};

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BillboardFoliageVertex {
    pub position: Vec3<f32>,
    pub uv0: Vec2<f32>,
    pub dimensions: Vec2<f32>,
}

impl Vertex for BillboardFoliageVertex {
    type VertexArgs = ();
}

impl From<GenericVertex> for BillboardFoliageVertex {
    #[inline]
    fn from(value: GenericVertex) -> Self {
        Self {
            position: value.position,
            uv0: value.uv0,
            dimensions: value.uv1,
        }
    }
}

impl From<BillboardFoliageVertex> for GenericVertex {
    #[inline]
    fn from(value: BillboardFoliageVertex) -> Self {
        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.dimensions,
            ..Default::default()
        }
    }
}

use binrw::{binrw, BinRead, BinWrite};

use crate::{
    math::{Vec2, Vec3, Vec4},
    render_block_model::{PackedRGBAU32, PackedVec4F32},
};

use super::{GenericVertex, Vertex};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HaloVertex {
    pub position: Vec3<f32>,
    pub color: Vec4<f32>,
    pub uv0: Vec2<f32>,
    pub dimensions: Vec2<f32>,
}

impl Vertex for HaloVertex {
    type VertexArgs = ();
}

impl BinRead for HaloVertex {
    type Args<'a> = <HaloVertex as Vertex>::VertexArgs;

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(PackedHaloVertex::read_options(reader, endian, ())?.into())
    }
}

impl BinWrite for HaloVertex {
    type Args<'a> = <HaloVertex as Vertex>::VertexArgs;

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        PackedHaloVertex::from(self.clone()).write_options(writer, endian, ())?;
        Ok(())
    }
}

impl From<PackedHaloVertex> for HaloVertex {
    #[inline]
    fn from(value: PackedHaloVertex) -> Self {
        let data: Vec4<f32> = value.data.into();
        Self {
            position: value.position,
            color: value.color.into(),
            uv0: Vec2::new(data.x, data.y),
            dimensions: Vec2::new(data.z, data.w),
        }
    }
}

impl From<GenericVertex> for HaloVertex {
    #[inline]
    fn from(value: GenericVertex) -> Self {
        Self {
            position: value.position,
            color: value.diffuse_color,
            uv0: value.uv0,
            dimensions: value.uv1,
        }
    }
}

impl From<HaloVertex> for GenericVertex {
    #[inline]
    fn from(value: HaloVertex) -> Self {
        Self {
            position: value.position,
            diffuse_color: value.color,
            uv0: value.uv0,
            uv1: value.dimensions,
            ..Default::default()
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PackedHaloVertex {
    pub position: Vec3<f32>,
    pub color: PackedRGBAU32,
    pub data: PackedVec4F32,
}

impl From<HaloVertex> for PackedHaloVertex {
    #[inline]
    fn from(value: HaloVertex) -> Self {
        Self {
            position: value.position,
            color: value.color.into(),
            data: Vec4::new(
                value.uv0.x,
                value.uv0.y,
                value.dimensions.x,
                value.dimensions.y,
            )
            .into(),
        }
    }
}

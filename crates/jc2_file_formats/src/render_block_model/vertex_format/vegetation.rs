use binrw::{BinRead, BinWrite, binrw};

use crate::{
    math::{Vec2, Vec3},
    render_block_model::{PackedNormalF32, PackedRGB},
};

use super::{GenericVertex, Vertex};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VegetationVertex {
    pub position: Vec3<f32>,
    pub uv0: Vec2<f32>,
    pub uv1: Vec2<f32>,
    pub color: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub tangent: Vec3<f32>,
    pub binormal: Vec3<f32>,
}

impl Vertex for VegetationVertex {
    type VertexArgs = (bool,);
}

impl BinRead for VegetationVertex {
    type Args<'a> = (bool,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(match args {
            (true,) => PackedBarkVertex::read_options(reader, endian, ())?.into(),
            (false,) => PackedTreeVertex::read_options(reader, endian, ())?.into(),
        })
    }
}

impl BinWrite for VegetationVertex {
    type Args<'a> = (bool,);

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let value = self.clone();
        match args {
            (true,) => PackedBarkVertex::from(value).write_options(writer, endian, ()),
            (false,) => PackedTreeVertex::from(value).write_options(writer, endian, ()),
        }
    }
}

impl From<PackedBarkVertex> for VegetationVertex {
    #[inline]
    fn from(value: PackedBarkVertex) -> Self {
        Self {
            position: value.position,
            uv0: value.uv0,
            color: value.color.into(),
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            ..Default::default()
        }
    }
}

impl From<PackedTreeVertex> for VegetationVertex {
    #[inline]
    fn from(value: PackedTreeVertex) -> Self {
        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            binormal: value.binormal.into(),
            ..Default::default()
        }
    }
}

impl From<GenericVertex> for VegetationVertex {
    #[inline]
    fn from(value: GenericVertex) -> Self {
        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            color: value.diffuse_color.into(),
            normal: value.normal,
            tangent: value.tangent,
            binormal: value.binormal,
        }
    }
}

impl From<VegetationVertex> for GenericVertex {
    #[inline]
    fn from(value: VegetationVertex) -> Self {
        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            diffuse_color: value.color.into(),
            normal: value.normal,
            tangent: value.tangent,
            binormal: value.binormal,
            ..Default::default()
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PackedBarkVertex {
    pub position: Vec3<f32>,
    pub color: PackedRGB,
    pub normal: PackedNormalF32,
    pub tangent: PackedNormalF32,
    pub uv0: Vec2<f32>,
}

impl From<VegetationVertex> for PackedBarkVertex {
    #[inline]
    fn from(value: VegetationVertex) -> Self {
        Self {
            position: value.position,
            color: value.color.into(),
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            uv0: value.uv0,
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PackedTreeVertex {
    pub position: Vec3<f32>,
    pub uv0: Vec2<f32>,
    pub uv1: Vec2<f32>,
    pub normal: PackedNormalF32,
    pub tangent: PackedNormalF32,
    pub binormal: PackedNormalF32,
}

impl From<VegetationVertex> for PackedTreeVertex {
    #[inline]
    fn from(value: VegetationVertex) -> Self {
        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            binormal: value.binormal.into(),
        }
    }
}

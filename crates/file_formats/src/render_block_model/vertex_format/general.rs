use binrw::{binrw, BinRead, BinWrite};

use crate::{
    math::{
        ops::{VecCross, VecDot},
        Vec2, Vec3, Vec4,
    },
    render_block_model::{PackedNormalF32, PackedPosition, PackedRGBAF32, PackedUV, VertexFormat},
};

use super::{GenericVertex, Vertex};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeneralVertex {
    pub position: Vec3<f32>,
    pub uv0: Vec2<f32>,
    pub uv1: Vec2<f32>,
    pub normal: Vec3<f32>,
    pub tangent: Vec3<f32>,
    pub color: Vec4<f32>,
}

impl Vertex for GeneralVertex {
    type VertexArgs = (VertexFormat,);
}

impl BinRead for GeneralVertex {
    type Args<'a> = (VertexFormat,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(match args {
            (VertexFormat::F32,) => GeneralVertexF32::read_options(reader, endian, ())?.into(),
            (VertexFormat::I16,) => GeneralVertexI16::read_options(reader, endian, ())?.into(),
        })
    }
}

impl BinWrite for GeneralVertex {
    type Args<'a> = (VertexFormat,);

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let value = self.clone();
        match args {
            (VertexFormat::F32,) => GeneralVertexF32::from(value).write_options(writer, endian, ()),
            (VertexFormat::I16,) => GeneralVertexI16::from(value).write_options(writer, endian, ()),
        }
    }
}

impl From<GeneralVertexF32> for GeneralVertex {
    #[inline]
    fn from(value: GeneralVertexF32) -> Self {
        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            color: value.color.into(),
        }
    }
}

impl From<GeneralVertexI16> for GeneralVertex {
    #[inline]
    fn from(value: GeneralVertexI16) -> Self {
        Self {
            position: value.position.into(),
            uv0: value.uv0.into(),
            uv1: value.uv1.into(),
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            color: value.color.into(),
        }
    }
}

impl From<GenericVertex> for GeneralVertex {
    #[inline]
    fn from(value: GenericVertex) -> Self {
        let sign = value
            .normal
            .cross(value.tangent)
            .dot(value.binormal)
            .signum();
        let tangent = value.tangent * sign;

        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            normal: value.normal,
            tangent,
            color: value.diffuse_color,
        }
    }
}

impl From<GeneralVertex> for GenericVertex {
    #[inline]
    fn from(value: GeneralVertex) -> Self {
        let normal: Vec3<f32> = value.normal;
        let tangent: Vec3<f32> = value.tangent;
        let binormal = normal.cross(tangent);

        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            normal,
            tangent,
            binormal,
            diffuse_color: value.color,
            ..Default::default()
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeneralVertexF32 {
    pub position: Vec3<f32>,
    pub uv0: Vec2<f32>,
    pub uv1: Vec2<f32>,
    pub normal: PackedNormalF32,
    pub tangent: PackedNormalF32,
    pub color: PackedRGBAF32,
}

impl From<GeneralVertex> for GeneralVertexF32 {
    #[inline]
    fn from(value: GeneralVertex) -> Self {
        value.into()
    }
}

impl From<GeneralVertexF32> for GenericVertex {
    #[inline]
    fn from(value: GeneralVertexF32) -> Self {
        GeneralVertex::from(value).into()
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeneralVertexI16 {
    pub uv0: PackedUV,
    pub uv1: PackedUV,
    pub normal: PackedNormalF32,
    pub tangent: PackedNormalF32,
    pub color: PackedRGBAF32,
    #[brw(pad_after = 2)]
    pub position: PackedPosition,
}

impl From<GeneralVertex> for GeneralVertexI16 {
    #[inline]
    fn from(value: GeneralVertex) -> Self {
        value.into()
    }
}

impl From<GeneralVertexI16> for GenericVertex {
    #[inline]
    fn from(value: GeneralVertexI16) -> Self {
        GeneralVertex::from(value).into()
    }
}

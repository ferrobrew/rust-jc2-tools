use binrw::{BinRead, BinWrite, binrw};

use crate::{
    math::{
        Vec2, Vec3, Vec4,
        ops::{VecCross, VecDot},
    },
    render_block_model::{
        PackedNormalF32, PackedPosition, PackedRGBAF32, PackedTangentF32, PackedUVF32, VertexFormat,
    },
};

use super::{GenericVertex, Vertex};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FacadeVertex {
    pub position: Vec3<f32>,
    pub uv0: Vec2<f32>,
    pub uv1: Vec2<f32>,
    pub normal: Vec3<f32>,
    pub tangent: Vec4<f32>,
    pub color: Vec4<f32>,
    pub uv2: Vec2<f32>,
}

impl Vertex for FacadeVertex {
    type VertexArgs = (VertexFormat,);
}

impl BinRead for FacadeVertex {
    type Args<'a> = (VertexFormat,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(match args {
            (VertexFormat::F32,) => FacadeVertexF32::read_options(reader, endian, ())?.into(),
            (VertexFormat::I16,) => FacadeVertexI16::read_options(reader, endian, ())?.into(),
        })
    }
}

impl BinWrite for FacadeVertex {
    type Args<'a> = (VertexFormat,);

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let value = self.clone();
        match args {
            (VertexFormat::F32,) => FacadeVertexF32::from(value).write_options(writer, endian, ()),
            (VertexFormat::I16,) => FacadeVertexI16::from(value).write_options(writer, endian, ()),
        }
    }
}

impl From<FacadeVertexF32> for FacadeVertex {
    #[inline]
    fn from(value: FacadeVertexF32) -> Self {
        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            color: value.color.into(),
            uv2: value.uv2.into(),
        }
    }
}

impl From<FacadeVertexI16> for FacadeVertex {
    #[inline]
    fn from(value: FacadeVertexI16) -> Self {
        Self {
            position: value.position.into(),
            uv0: value.uv0,
            uv1: value.uv1,
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            color: value.color.into(),
            uv2: value.uv2.into(),
        }
    }
}

impl From<GenericVertex> for FacadeVertex {
    #[inline]
    fn from(value: GenericVertex) -> Self {
        let sign = value
            .normal
            .cross(value.tangent)
            .dot(value.binormal)
            .signum();

        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            normal: value.normal,
            tangent: value.tangent.extend(sign),
            color: value.diffuse_color,
            uv2: value.uv2,
        }
    }
}

impl From<FacadeVertex> for GenericVertex {
    #[inline]
    fn from(value: FacadeVertex) -> Self {
        let normal: Vec3<f32> = value.normal;
        let tangent: Vec3<f32> = value.tangent.into();
        let binormal = normal.cross(tangent) * value.tangent.w;

        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            uv2: value.uv2,
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
pub struct FacadeVertexF32 {
    pub position: Vec3<f32>,
    pub uv0: Vec2<f32>,
    pub uv1: Vec2<f32>,
    pub normal: PackedNormalF32,
    pub tangent: PackedTangentF32,
    pub color: PackedRGBAF32,
    pub uv2: PackedUVF32,
}

impl From<FacadeVertex> for FacadeVertexF32 {
    #[inline]
    fn from(value: FacadeVertex) -> Self {
        Self {
            position: value.position,
            uv0: value.uv0,
            uv1: value.uv1,
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            color: value.color.into(),
            uv2: value.uv2.into(),
        }
    }
}

impl From<FacadeVertexF32> for GenericVertex {
    #[inline]
    fn from(value: FacadeVertexF32) -> Self {
        FacadeVertex::from(value).into()
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FacadeVertexI16 {
    pub position: PackedPosition,
    #[brw(magic(32767u16))]
    pub uv0: Vec2<f32>,
    pub uv1: Vec2<f32>,
    pub normal: PackedNormalF32,
    pub tangent: PackedTangentF32,
    pub color: PackedRGBAF32,
    pub uv2: PackedUVF32,
}

impl From<FacadeVertex> for FacadeVertexI16 {
    #[inline]
    fn from(value: FacadeVertex) -> Self {
        Self {
            position: value.position.into(),
            uv0: value.uv0,
            uv1: value.uv1,
            normal: value.normal.into(),
            tangent: value.tangent.into(),
            color: value.color.into(),
            uv2: value.uv2.into(),
        }
    }
}

impl From<FacadeVertexI16> for GenericVertex {
    #[inline]
    fn from(value: FacadeVertexI16) -> Self {
        FacadeVertex::from(value).into()
    }
}

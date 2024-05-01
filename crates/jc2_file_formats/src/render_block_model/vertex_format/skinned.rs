use binrw::{binrw, BinRead, BinWrite};

use crate::{
    math::{Vec2, Vec3},
    render_block_model::{PackedNormalU32, RenderBlockError},
};

use super::{GenericVertex, Vertex, VertexBuffer};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinnedVertex {
    pub position: Vec3<f32>,
    pub bone_weights: [f32; 8],
    pub bone_indices: [u32; 8],
    pub normal: Vec3<f32>,
    pub tangent: Vec3<f32>,
    pub binormal: Vec3<f32>,
    pub uv0: Vec2<f32>,
}

impl Vertex for SkinnedVertex {
    type VertexArgs = (bool,);
}

type BinError = binrw::Error;
type SkinnedPositions = VertexBuffer<SkinnedVertexPosition>;
type SkinnedData = VertexBuffer<SkinnedVertexData>;

impl BinRead for VertexBuffer<SkinnedVertex> {
    type Args<'a> = <SkinnedVertex as Vertex>::VertexArgs;

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let positions = SkinnedPositions::read_options(reader, endian, args)?;
        let datas = SkinnedData::read_options(reader, endian, ())?;

        if positions.len() == datas.len() {
            let mut vertices = Vec::with_capacity(positions.len());
            for (position, data) in positions.iter().zip(datas.iter()) {
                vertices.push(SkinnedVertex {
                    position: position.position,
                    bone_weights: position.bone_weights,
                    bone_indices: position.bone_indices,
                    normal: data.normal.into(),
                    tangent: data.tangent.into(),
                    binormal: data.binormal.into(),
                    uv0: data.uv0,
                });
            }
            Ok(Self(vertices))
        } else {
            Err(BinError::Custom {
                pos: reader.stream_position()?,
                err: Box::new(RenderBlockError::InvalidArrayLength),
            })
        }
    }
}

impl BinWrite for VertexBuffer<SkinnedVertex> {
    type Args<'a> = <SkinnedVertex as Vertex>::VertexArgs;

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let mut positions = Vec::with_capacity(self.len());
        let mut datas = Vec::with_capacity(self.len());
        for vertex in self.iter() {
            positions.push(SkinnedVertexPosition {
                position: vertex.position,
                bone_weights: vertex.bone_weights,
                bone_indices: vertex.bone_indices,
            });
            datas.push(SkinnedVertexData {
                normal: vertex.normal.into(),
                tangent: vertex.tangent.into(),
                binormal: vertex.binormal.into(),
                uv0: vertex.uv0,
            });
        }
        positions.write_options(writer, endian, args)?;
        datas.write_options(writer, endian, ())?;
        Ok(())
    }
}

impl From<GenericVertex> for SkinnedVertex {
    #[inline]
    fn from(value: GenericVertex) -> Self {
        Self {
            position: value.position,
            bone_weights: value.bone_weights,
            bone_indices: value.bone_indices,
            normal: value.normal,
            tangent: value.tangent,
            binormal: value.binormal,
            uv0: value.uv0,
        }
    }
}

impl From<SkinnedVertex> for GenericVertex {
    #[inline]
    fn from(value: SkinnedVertex) -> Self {
        Self {
            position: value.position,
            bone_weights: value.bone_weights,
            bone_indices: value.bone_indices,
            normal: value.normal,
            tangent: value.tangent,
            binormal: value.binormal,
            uv0: value.uv0,
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinnedVertexPosition {
    pub position: Vec3<f32>,
    pub bone_weights: [f32; 8],
    pub bone_indices: [u32; 8],
}

impl Vertex for SkinnedVertexPosition {
    type VertexArgs = (bool,);
}

impl BinRead for SkinnedVertexPosition {
    type Args<'a> = (bool,);

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(match args {
            (false,) => SkinnedVertex4Position::read_options(reader, endian, ())?.into(),
            (true,) => SkinnedVertex8Position::read_options(reader, endian, ())?.into(),
        })
    }
}

impl BinWrite for SkinnedVertexPosition {
    type Args<'a> = (bool,);

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let value = self.clone();
        match args {
            (false,) => SkinnedVertex4Position::from(value).write_options(writer, endian, ()),
            (true,) => SkinnedVertex8Position::from(value).write_options(writer, endian, ()),
        }
    }
}

impl From<SkinnedVertex4Position> for SkinnedVertexPosition {
    #[inline]
    fn from(value: SkinnedVertex4Position) -> Self {
        let bone_weights: [u8; 4] = bytemuck::must_cast(value.bone_weights);
        let bone_indices: [u8; 4] = bytemuck::must_cast(value.bone_indices);
        Self {
            position: value.position,
            bone_weights: [
                bone_weights[0] as f32 / 255.0,
                bone_weights[1] as f32 / 255.0,
                bone_weights[2] as f32 / 255.0,
                bone_weights[3] as f32 / 255.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            bone_indices: [
                bone_indices[0] as u32,
                bone_indices[1] as u32,
                bone_indices[2] as u32,
                bone_indices[3] as u32,
                0,
                0,
                0,
                0,
            ],
        }
    }
}

impl From<SkinnedVertex8Position> for SkinnedVertexPosition {
    #[inline]
    fn from(value: SkinnedVertex8Position) -> Self {
        let bone_weights: [u8; 8] = bytemuck::must_cast(value.bone_weights);
        let bone_indices: [u8; 8] = bytemuck::must_cast(value.bone_indices);
        Self {
            position: value.position,
            bone_weights: [
                bone_weights[0] as f32 / 255.0,
                bone_weights[1] as f32 / 255.0,
                bone_weights[2] as f32 / 255.0,
                bone_weights[3] as f32 / 255.0,
                bone_weights[4] as f32 / 255.0,
                bone_weights[5] as f32 / 255.0,
                bone_weights[6] as f32 / 255.0,
                bone_weights[7] as f32 / 255.0,
            ],
            bone_indices: [
                bone_indices[0] as u32,
                bone_indices[1] as u32,
                bone_indices[2] as u32,
                bone_indices[3] as u32,
                bone_indices[4] as u32,
                bone_indices[5] as u32,
                bone_indices[6] as u32,
                bone_indices[7] as u32,
            ],
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinnedVertex4Position {
    pub position: Vec3<f32>,
    pub bone_weights: u32,
    pub bone_indices: u32,
}

impl From<SkinnedVertexPosition> for SkinnedVertex4Position {
    #[inline]
    fn from(value: SkinnedVertexPosition) -> Self {
        let bone_weights: u32 = bytemuck::must_cast([
            (value.bone_weights[0] * 255.0) as u8,
            (value.bone_weights[1] * 255.0) as u8,
            (value.bone_weights[2] * 255.0) as u8,
            (value.bone_weights[3] * 255.0) as u8,
        ]);
        let bone_indices: u32 = bytemuck::must_cast([
            value.bone_weights[0] as u8,
            value.bone_weights[1] as u8,
            value.bone_weights[2] as u8,
            value.bone_weights[3] as u8,
        ]);
        Self {
            position: value.position,
            bone_weights,
            bone_indices,
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinnedVertex8Position {
    pub position: Vec3<f32>,
    pub bone_weights: [u32; 2],
    pub bone_indices: [u32; 2],
}

impl From<SkinnedVertexPosition> for SkinnedVertex8Position {
    #[inline]
    fn from(value: SkinnedVertexPosition) -> Self {
        let bone_weights: [u32; 2] = bytemuck::must_cast([
            (value.bone_weights[0] * 255.0) as u8,
            (value.bone_weights[1] * 255.0) as u8,
            (value.bone_weights[2] * 255.0) as u8,
            (value.bone_weights[3] * 255.0) as u8,
            (value.bone_weights[4] * 255.0) as u8,
            (value.bone_weights[5] * 255.0) as u8,
            (value.bone_weights[6] * 255.0) as u8,
            (value.bone_weights[7] * 255.0) as u8,
        ]);
        let bone_indices: [u32; 2] = bytemuck::must_cast([
            value.bone_weights[0] as u8,
            value.bone_weights[1] as u8,
            value.bone_weights[2] as u8,
            value.bone_weights[3] as u8,
            value.bone_weights[4] as u8,
            value.bone_weights[5] as u8,
            value.bone_weights[6] as u8,
            value.bone_weights[7] as u8,
        ]);
        Self {
            position: value.position,
            bone_weights,
            bone_indices,
        }
    }
}

#[binrw]
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkinnedVertexData {
    pub normal: PackedNormalU32,
    pub tangent: PackedNormalU32,
    pub binormal: PackedNormalU32,
    pub uv0: Vec2<f32>,
}

impl Vertex for SkinnedVertexData {
    type VertexArgs = ();
}

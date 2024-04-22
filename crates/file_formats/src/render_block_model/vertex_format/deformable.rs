use binrw::{binrw, BinRead, BinWrite};

use crate::{
    math::{Vec2, Vec3},
    render_block_model::{PackedNormalF32, RenderBlockError},
};

use super::{GenericVertex, Vertex, VertexBuffer};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeformableVertex {
    pub position: Vec3<f32>,
    pub morph_position: Vec3<f32>,
    pub bone_weights: [f32; 4],
    pub bone_indices: [u32; 4],
    pub normal: Vec3<f32>,
    pub morph_normal: Vec3<f32>,
    pub tangent: Vec3<f32>,
    pub morph_tangent: Vec3<f32>,
    pub uv0: Vec2<f32>,
    pub light: f32,
}

impl Vertex for DeformableVertex {
    type VertexArgs = (bool,);
}

type BinError = binrw::Error;
type DeformablePacked = VertexBuffer<DeformableVertexPacked>;
type DeformablePositions = VertexBuffer<DeformableVertexPosition>;
type DeformableData = VertexBuffer<DeformableVertexData>;

impl BinRead for VertexBuffer<DeformableVertex> {
    type Args<'a> = <DeformableVertex as Vertex>::VertexArgs;

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        if args.0 {
            let positions = DeformablePositions::read_options(reader, endian, ())?;
            let datas = DeformableData::read_options(reader, endian, ())?;

            if positions.len() == datas.len() {
                let mut vertices = Vec::with_capacity(positions.len());
                for (position, data) in positions.iter().zip(datas.iter()) {
                    vertices.push(DeformableVertexPacked {
                        position: position.position,
                        morph_position: position.morph_position,
                        bone_weights_indices: position.bone_weights_indices,
                        uv0: data.uv0,
                        light: data.light,
                        normal: data.normal,
                        morph_normal: data.morph_normal,
                        tangent: data.tangent,
                        morph_tangent: data.morph_tangent,
                    })
                }
                Ok(Self(
                    vertices.into_iter().map(DeformableVertex::from).collect(),
                ))
            } else {
                Err(BinError::Custom {
                    pos: reader.stream_position()?,
                    err: Box::new(RenderBlockError::InvalidArrayLength),
                })
            }
        } else {
            Ok(Self(
                DeformablePacked::read_options(reader, endian, ())?
                    .0
                    .into_iter()
                    .map(DeformableVertex::from)
                    .collect(),
            ))
        }
    }
}

impl BinWrite for VertexBuffer<DeformableVertex> {
    type Args<'a> = <DeformableVertex as Vertex>::VertexArgs;

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let vertices: Vec<DeformableVertexPacked> = self
            .0
            .clone()
            .into_iter()
            .map(DeformableVertexPacked::from)
            .collect();
        if args.0 {
            let mut positions = Vec::with_capacity(vertices.len());
            let mut datas = Vec::with_capacity(vertices.len());
            for vertex in vertices.iter() {
                positions.push(DeformableVertexPosition {
                    position: vertex.position,
                    morph_position: vertex.morph_position,
                    bone_weights_indices: vertex.bone_weights_indices,
                });
                datas.push(DeformableVertexData {
                    uv0: vertex.uv0,
                    light: vertex.light,
                    normal: vertex.normal,
                    morph_normal: vertex.morph_normal,
                    tangent: vertex.tangent,
                    morph_tangent: vertex.morph_tangent,
                });
            }
            positions.write_options(writer, endian, ())?;
            datas.write_options(writer, endian, ())?;
        } else {
            vertices.write_options(writer, endian, ())?;
        }
        Ok(())
    }
}

impl From<DeformableVertexPacked> for DeformableVertex {
    fn from(value: DeformableVertexPacked) -> Self {
        Self {
            position: value.position,
            morph_position: value.position + (Vec3::from(value.morph_position) * 2.5),
            bone_weights: [
                (value.bone_weights_indices[0] & 0xFF) as f32 / 255.0,
                (value.bone_weights_indices[1] & 0xFF) as f32 / 255.0,
                (value.bone_weights_indices[2] & 0xFF) as f32 / 255.0,
                (value.bone_weights_indices[3] & 0xFF) as f32 / 255.0,
            ],
            bone_indices: [
                (((value.bone_weights_indices[0] >> 8) & 0xFF) + 128) as u32,
                (((value.bone_weights_indices[1] >> 8) & 0xFF) + 128) as u32,
                (((value.bone_weights_indices[2] >> 8) & 0xFF) + 128) as u32,
                (((value.bone_weights_indices[3] >> 8) & 0xFF) + 128) as u32,
            ],
            uv0: value.uv0,
            light: value.light,
            normal: value.normal.into(),
            morph_normal: value.morph_normal.into(),
            tangent: value.tangent.into(),
            morph_tangent: value.morph_tangent.into(),
        }
    }
}

impl From<GenericVertex> for DeformableVertex {
    #[inline]
    fn from(value: GenericVertex) -> Self {
        value.into()
    }
}

impl From<DeformableVertex> for GenericVertex {
    #[inline]
    fn from(value: DeformableVertex) -> Self {
        Self {
            position: value.position,
            normal: value.normal,
            tangent: value.tangent,
            morph_position: value.morph_position,
            morph_normal: value.morph_normal,
            morph_tangent: value.morph_tangent,
            uv0: value.uv0,
            size: value.light,
            bone_weights: [
                value.bone_weights[0],
                value.bone_weights[1],
                value.bone_weights[2],
                value.bone_weights[3],
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            bone_indices: [
                value.bone_indices[0],
                value.bone_indices[1],
                value.bone_indices[2],
                value.bone_indices[3],
                0,
                0,
                0,
                0,
            ],
            ..Default::default()
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeformableVertexPacked {
    pub position: Vec3<f32>,
    pub morph_position: PackedNormalF32,
    pub bone_weights_indices: [u16; 4],
    pub uv0: Vec2<f32>,
    pub light: f32,
    pub normal: PackedNormalF32,
    pub morph_normal: PackedNormalF32,
    pub tangent: PackedNormalF32,
    pub morph_tangent: PackedNormalF32,
}

impl Vertex for DeformableVertexPacked {
    type VertexArgs = ();
}

impl From<DeformableVertex> for DeformableVertexPacked {
    fn from(value: DeformableVertex) -> Self {
        Self {
            position: value.position,
            morph_position: ((value.morph_position - value.position) / 2.5).into(),
            bone_weights_indices: [
                ((value.bone_weights[0] * 255.0) as u16 & 0xFF)
                    | (((value.bone_indices[0] - 128) & 0xFF) << 8) as u16,
                ((value.bone_weights[1] * 255.0) as u16 & 0xFF)
                    | (((value.bone_indices[1] - 128) & 0xFF) << 8) as u16,
                ((value.bone_weights[2] * 255.0) as u16 & 0xFF)
                    | (((value.bone_indices[2] - 128) & 0xFF) << 8) as u16,
                ((value.bone_weights[3] * 255.0) as u16 & 0xFF)
                    | (((value.bone_indices[3] - 128) & 0xFF) << 8) as u16,
            ],
            uv0: value.uv0,
            light: value.light,
            normal: value.normal.into(),
            morph_normal: value.morph_normal.into(),
            tangent: value.tangent.into(),
            morph_tangent: value.morph_tangent.into(),
        }
    }
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeformableVertexPosition {
    pub position: Vec3<f32>,
    pub morph_position: PackedNormalF32,
    pub bone_weights_indices: [u16; 4],
}

impl Vertex for DeformableVertexPosition {
    type VertexArgs = ();
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeformableVertexData {
    pub uv0: Vec2<f32>,
    pub light: f32,
    pub normal: PackedNormalF32,
    pub morph_normal: PackedNormalF32,
    pub tangent: PackedNormalF32,
    pub morph_tangent: PackedNormalF32,
}

impl Vertex for DeformableVertexData {
    type VertexArgs = ();
}

use binrw::{binrw, BinRead, BinWrite};

use crate::{
    math::{Vec2, Vec3},
    render_block_model::{PackedNormalF32, PackedWeightAndIndex, RenderBlockError},
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
}

impl Vertex for DeformableVertex {
    type VertexArgs = ();
}

impl BinRead for DeformableVertex {
    type Args<'a> = <DeformableVertex as Vertex>::VertexArgs;

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        Ok(PackedDeformableVertex::read_options(reader, endian, ())?.into())
    }
}

impl BinWrite for DeformableVertex {
    type Args<'a> = <DeformableVertex as Vertex>::VertexArgs;

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        PackedDeformableVertex::from(self.clone()).write_options(writer, endian, ())?;
        Ok(())
    }
}

impl From<PackedDeformableVertex> for DeformableVertex {
    fn from(value: PackedDeformableVertex) -> Self {
        Self {
            position: value.position,
            morph_position: Vec3::from(value.morph_position) * 2.5,
            bone_weights: [
                value.bone_weights_indices[0].weight(),
                value.bone_weights_indices[1].weight(),
                value.bone_weights_indices[2].weight(),
                value.bone_weights_indices[3].weight(),
            ],
            bone_indices: [
                value.bone_weights_indices[0].index(),
                value.bone_weights_indices[1].index(),
                value.bone_weights_indices[2].index(),
                value.bone_weights_indices[3].index(),
            ],
            uv0: value.uv0,
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
        Self {
            position: value.position,
            morph_position: value.morph_position,
            bone_weights: [
                value.bone_weights[0],
                value.bone_weights[1],
                value.bone_weights[2],
                value.bone_weights[3],
            ],
            bone_indices: [
                value.bone_indices[0],
                value.bone_indices[1],
                value.bone_indices[2],
                value.bone_indices[3],
            ],
            uv0: value.uv0,
            normal: value.normal,
            morph_normal: value.morph_normal,
            tangent: value.tangent,
            morph_tangent: value.morph_tangent,
        }
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
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PackedDeformableVertex {
    pub position: Vec3<f32>,
    pub morph_position: PackedNormalF32,
    pub bone_weights_indices: [PackedWeightAndIndex; 4],
    pub normal: PackedNormalF32,
    pub morph_normal: PackedNormalF32,
    pub tangent: PackedNormalF32,
    pub morph_tangent: PackedNormalF32,
    pub uv0: Vec2<f32>,
}

impl Vertex for PackedDeformableVertex {
    type VertexArgs = ();
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LitDeformableVertex {
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

impl Vertex for LitDeformableVertex {
    type VertexArgs = (bool,);
}

type BinError = binrw::Error;
type PackedLitDeformableVertices = VertexBuffer<PackedLitDeformableVertex>;
type DeformablePositions = VertexBuffer<DeformableVertexPosition>;
type LitDeformableData = VertexBuffer<LitDeformableVertexData>;

impl BinRead for VertexBuffer<LitDeformableVertex> {
    type Args<'a> = <LitDeformableVertex as Vertex>::VertexArgs;

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        if args.0 {
            let positions = DeformablePositions::read_options(reader, endian, ())?;
            let datas = LitDeformableData::read_options(reader, endian, ())?;

            if positions.len() == datas.len() {
                let mut vertices = Vec::with_capacity(positions.len());
                for (position, data) in positions.iter().zip(datas.iter()) {
                    vertices.push(PackedLitDeformableVertex {
                        position: position.position,
                        morph_position: position.morph_position,
                        bone_weights_indices: position.bone_weights_indices,
                        uv0: data.uv0,
                        light: data.light,
                        normal: data.normal,
                        morph_normal: data.morph_normal,
                        tangent: data.tangent,
                        morph_tangent: data.morph_tangent,
                    });
                }
                Ok(Self(
                    vertices
                        .into_iter()
                        .map(LitDeformableVertex::from)
                        .collect(),
                ))
            } else {
                Err(BinError::Custom {
                    pos: reader.stream_position()?,
                    err: Box::new(RenderBlockError::InvalidArrayLength),
                })
            }
        } else {
            Ok(Self(
                PackedLitDeformableVertices::read_options(reader, endian, ())?
                    .0
                    .into_iter()
                    .map(LitDeformableVertex::from)
                    .collect(),
            ))
        }
    }
}

impl BinWrite for VertexBuffer<LitDeformableVertex> {
    type Args<'a> = <LitDeformableVertex as Vertex>::VertexArgs;

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        let vertices: Vec<PackedLitDeformableVertex> = self
            .0
            .clone()
            .into_iter()
            .map(PackedLitDeformableVertex::from)
            .collect();
        if args.0 {
            let mut positions = Vec::with_capacity(vertices.len());
            let mut datas = Vec::with_capacity(vertices.len());
            for vertex in &vertices {
                positions.push(DeformableVertexPosition {
                    position: vertex.position,
                    morph_position: vertex.morph_position,
                    bone_weights_indices: vertex.bone_weights_indices,
                });
                datas.push(LitDeformableVertexData {
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

impl From<PackedLitDeformableVertex> for LitDeformableVertex {
    fn from(value: PackedLitDeformableVertex) -> Self {
        Self {
            position: value.position,
            morph_position: Vec3::from(value.morph_position) * 2.5,
            bone_weights: [
                value.bone_weights_indices[0].weight(),
                value.bone_weights_indices[1].weight(),
                value.bone_weights_indices[2].weight(),
                value.bone_weights_indices[3].weight(),
            ],
            bone_indices: [
                value.bone_weights_indices[0].index(),
                value.bone_weights_indices[1].index(),
                value.bone_weights_indices[2].index(),
                value.bone_weights_indices[3].index(),
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

impl From<GenericVertex> for LitDeformableVertex {
    #[inline]
    fn from(value: GenericVertex) -> Self {
        Self {
            position: value.position,
            normal: value.normal,
            tangent: value.tangent,
            morph_position: value.morph_position,
            morph_normal: value.morph_normal,
            morph_tangent: value.morph_tangent,
            uv0: value.uv0,
            light: value.size,
            bone_weights: [
                value.bone_weights[0],
                value.bone_weights[1],
                value.bone_weights[2],
                value.bone_weights[3],
            ],
            bone_indices: [
                value.bone_indices[0],
                value.bone_indices[1],
                value.bone_indices[2],
                value.bone_indices[3],
            ],
        }
    }
}

impl From<LitDeformableVertex> for GenericVertex {
    #[inline]
    fn from(value: LitDeformableVertex) -> Self {
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
pub struct PackedLitDeformableVertex {
    pub position: Vec3<f32>,
    pub morph_position: PackedNormalF32,
    pub bone_weights_indices: [PackedWeightAndIndex; 4],
    pub uv0: Vec2<f32>,
    pub light: f32,
    pub normal: PackedNormalF32,
    pub morph_normal: PackedNormalF32,
    pub tangent: PackedNormalF32,
    pub morph_tangent: PackedNormalF32,
}

impl Vertex for PackedLitDeformableVertex {
    type VertexArgs = ();
}

impl From<DeformableVertex> for PackedDeformableVertex {
    fn from(value: DeformableVertex) -> Self {
        Self {
            position: value.position,
            morph_position: (value.morph_position / 2.5).into(),
            bone_weights_indices: [
                PackedWeightAndIndex::new(value.bone_weights[0], value.bone_indices[0]),
                PackedWeightAndIndex::new(value.bone_weights[1], value.bone_indices[1]),
                PackedWeightAndIndex::new(value.bone_weights[2], value.bone_indices[2]),
                PackedWeightAndIndex::new(value.bone_weights[3], value.bone_indices[3]),
            ],
            uv0: value.uv0,
            normal: value.normal.into(),
            morph_normal: value.morph_normal.into(),
            tangent: value.tangent.into(),
            morph_tangent: value.morph_tangent.into(),
        }
    }
}

impl From<LitDeformableVertex> for PackedLitDeformableVertex {
    fn from(value: LitDeformableVertex) -> Self {
        Self {
            position: value.position,
            morph_position: (value.morph_position / 2.5).into(),
            bone_weights_indices: [
                PackedWeightAndIndex::new(value.bone_weights[0], value.bone_indices[0]),
                PackedWeightAndIndex::new(value.bone_weights[1], value.bone_indices[1]),
                PackedWeightAndIndex::new(value.bone_weights[2], value.bone_indices[2]),
                PackedWeightAndIndex::new(value.bone_weights[3], value.bone_indices[3]),
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
    pub bone_weights_indices: [PackedWeightAndIndex; 4],
}

impl Vertex for DeformableVertexPosition {
    type VertexArgs = ();
}

#[binrw]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LitDeformableVertexData {
    pub uv0: Vec2<f32>,
    pub light: f32,
    pub normal: PackedNormalF32,
    pub morph_normal: PackedNormalF32,
    pub tangent: PackedNormalF32,
    pub morph_tangent: PackedNormalF32,
}

impl Vertex for LitDeformableVertexData {
    type VertexArgs = ();
}

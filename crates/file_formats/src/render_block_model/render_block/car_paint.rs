use binrw::{binrw, BinRead, BinWrite};
use bitflags::bitflags;

use crate::{
    math::{Vec3, Vec4},
    render_block_model::{DeformTable, DeformableVertex, IndexBuffer, Material, VertexBuffer},
};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum CarPaintVersion {
    V1 = 1,
    V2,
    #[default]
    V3,
    V4,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
    pub struct CarPaintFlags: u32 {
        const NO_CULLING = 1 << 0;
        const ALPHA_BLENDING = 1 << 1;
        const IGNORE_PALETTE = 1 << 3;
        const NO_DIRT = 1 << 4;
        const DECAL = 1 << 5;
        const MASK_WATER = 1 << 6;
        const ALPHA_TEST = 1 << 7;
        const DULL = 1 << 8;
        const IS_LIGHT = 1 << 9;
        const FLAT_NORMAL = 1 << 10;
    }
}

#[binrw]
#[derive(Clone, Debug)]
pub struct CarPaintAttributes {
    pub two_tone_colors: [Vec3<f32>; 2],
    pub specular_power: f32,
    pub depth_bias: f32,
    pub reflection_multiplier: f32,
    pub noise_factors: Vec4<f32>,
    pub flags: CarPaintFlags,
}

impl Default for CarPaintAttributes {
    #[inline]
    fn default() -> Self {
        Self {
            two_tone_colors: [
                Vec3::new(0.784, 0.784, 0.784),
                Vec3::new(0.651, 0.729, 0.827),
            ],
            specular_power: 64.0,
            depth_bias: 0.0,
            reflection_multiplier: 0.5,
            noise_factors: Vec4::splat(0.0),
            flags: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CarPaintRenderBlock {
    pub version: CarPaintVersion,
    pub attributes: CarPaintAttributes,
    pub material: Material,
    pub vertices: VertexBuffer<DeformableVertex>,
    pub indices: IndexBuffer<u16>,
    pub deform_table: DeformTable,
}

impl BinRead for CarPaintRenderBlock {
    type Args<'a> = ();

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let mut result = Self {
            version: CarPaintVersion::read_options(reader, endian, ())?,
            attributes: CarPaintAttributes::read_options(reader, endian, ())?,
            ..Default::default()
        };
        if result.version != CarPaintVersion::V3 {
            result.deform_table = DeformTable::read_options(reader, endian, ())?;
        }
        result.material = Material::read_options(reader, endian, ())?;
        result.vertices = VertexBuffer::<DeformableVertex>::read_options(
            reader,
            endian,
            (result.version as u32 > 2,),
        )?;
        result.indices =
            IndexBuffer::<u16>::read_options(reader, endian, (result.vertices.len(),))?;
        if result.version == CarPaintVersion::V3 {
            result.deform_table = DeformTable::read_options(reader, endian, ())?;
        }
        Ok(result)
    }
}

impl BinWrite for CarPaintRenderBlock {
    type Args<'a> = ();

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        if self.version != CarPaintVersion::V3 {
            self.deform_table.write_options(writer, endian, args)?;
        }
        self.material.write_options(writer, endian, args)?;
        self.vertices
            .write_options(writer, endian, (self.version as u32 > 2,))?;
        self.indices
            .write_options(writer, endian, (self.vertices.len(),))?;
        if self.version == CarPaintVersion::V3 {
            self.deform_table.write_options(writer, endian, args)?;
        }
        Ok(())
    }
}

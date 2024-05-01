use binrw::{binrw, BinRead, BinWrite};
use bitflags::bitflags;

use crate::{
    math::Vec3,
    render_block_model::{IndexBuffer, Material, SkinBatch, SkinnedVertex, VertexBuffer},
};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum SkinnedGeneralVersion {
    V1 = 1,
    #[default]
    V3 = 3,
}

#[binrw]
#[brw(repr = u16)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum SkinnedGeneralTechnique {
    #[default]
    Skin,
    Hair,
    Cloth,
    Metal,
    EyeGloss,
    Unknown,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
    pub struct SkinnedGeneralFlags: u16 {
        const NO_CULLING = 1 << 0;
        const ALPHA_TEST = 1 << 1;
        const ALPHA_BLENDING = 1 << 2;
        const EIGHT_BONE_INFLUENCE = 1 << 3;
        const USE_SNOW_FLAG = 1 << 4;
    }
}
#[derive(Clone, Debug)]
pub struct SkinnedGeneralAttributes {
    pub technique: SkinnedGeneralTechnique,
    pub flags: SkinnedGeneralFlags,
    pub specular_power: f32,
    pub rim_intensity: f32,
    pub rim_power: f32,
    pub rim_weights: Vec3<f32>,
}

impl Default for SkinnedGeneralAttributes {
    #[inline]
    fn default() -> Self {
        Self {
            technique: SkinnedGeneralTechnique::Skin,
            flags: SkinnedGeneralFlags::USE_SNOW_FLAG,
            specular_power: 10.0,
            rim_intensity: 1.0,
            rim_power: 10.0,
            rim_weights: Vec3::splat(1.0),
        }
    }
}

impl BinRead for SkinnedGeneralAttributes {
    type Args<'a> = ();

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let (technique, flags) = match endian {
            binrw::Endian::Big => {
                let flags = SkinnedGeneralFlags::read_options(reader, endian, ())?;
                let technique = SkinnedGeneralTechnique::read_options(reader, endian, ())?;
                (technique, flags)
            }
            binrw::Endian::Little => (
                SkinnedGeneralTechnique::read_options(reader, endian, ())?,
                SkinnedGeneralFlags::read_options(reader, endian, ())?,
            ),
        };
        Ok(Self {
            technique,
            flags,
            specular_power: f32::read_options(reader, endian, ())?,
            rim_intensity: f32::read_options(reader, endian, ())?,
            rim_power: f32::read_options(reader, endian, ())?,
            rim_weights: Vec3::<f32>::read_options(reader, endian, ())?,
        })
    }
}

impl BinWrite for SkinnedGeneralAttributes {
    type Args<'a> = ();

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        match endian {
            binrw::Endian::Big => {
                self.flags.write_options(writer, endian, ())?;
                self.technique.write_options(writer, endian, ())?;
            }
            binrw::Endian::Little => {
                self.technique.write_options(writer, endian, ())?;
                self.flags.write_options(writer, endian, ())?;
            }
        };
        self.specular_power.write_options(writer, endian, ())?;
        self.rim_intensity.write_options(writer, endian, ())?;
        self.rim_power.write_options(writer, endian, ())?;
        self.rim_weights.write_options(writer, endian, ())?;
        Ok(())
    }
}

#[binrw]
#[derive(Clone, Debug, Default)]
pub struct SkinnedGeneralRenderBlock {
    pub version: SkinnedGeneralVersion,
    pub attributes: SkinnedGeneralAttributes,
    pub material: Material,
    #[brw(args(attributes.flags.intersects(SkinnedGeneralFlags::EIGHT_BONE_INFLUENCE)))]
    pub vertices: VertexBuffer<SkinnedVertex>,
    pub skin_batches: VertexBuffer<SkinBatch>,
    #[brw(args(vertices.len()))]
    pub indices: IndexBuffer<u16>,
}

use binrw::{binrw, BinRead, BinWrite};
use bitflags::bitflags;

use crate::render_block_model::{GeneralVertex, Indices, VertexInfo, Vertices};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum LambertVersion {
    V0,
    V1, // TODO: not sure this exists in the wild
    V2,
    V3,
    #[default]
    V4,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
    pub struct LambertFlags: u32 {
        const ALPHA_BLENDING = 1 << 0;
        const ALPHA_TEST = 1 << 1;
        const TWO_SIDED = 1 << 2;
        const FLAT_NORMAL = 1 << 3;
        const NO_DIRT = 1 << 4;
        const USE_SNOW = 1 << 5;
        const USE_DYNAMIC_LIGHTS = 1 << 6;
        const USE_CHANNEL_TEXTURES = 1 << 7;
        const USE_CHANNEL_AMBIENT_OCCLUSION = 1 << 8;
    }
}

#[derive(Clone, Debug)]
pub struct LambertAttributes {
    pub vertex_info: VertexInfo,
    pub flags: LambertFlags,
    pub depth_bias: f32,
    pub texture_channel: u8,
    pub ambient_occlusion_channel: u8,
}

impl Default for LambertAttributes {
    #[inline]
    fn default() -> Self {
        Self {
            vertex_info: Default::default(),
            flags: LambertFlags::USE_DYNAMIC_LIGHTS,
            depth_bias: 0.0,
            texture_channel: 0,
            ambient_occlusion_channel: 0,
        }
    }
}

impl BinRead for LambertAttributes {
    type Args<'a> = (&'a LambertVersion,);

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let mut result: Self = Default::default();

        if args.0 == &LambertVersion::V4 {
            result.vertex_info = VertexInfo::read_options(reader, endian, (false,))?;
        }

        result.flags = LambertFlags::read_options(reader, endian, ())?;

        if args.0 == &LambertVersion::V0 {
            result.flags &= LambertFlags::USE_DYNAMIC_LIGHTS;
        }

        if args.0 == &LambertVersion::V3 {
            result.vertex_info = VertexInfo::read_options(reader, endian, (false,))?;
        }

        if args.0 == &LambertVersion::V4 {
            result.texture_channel = u8::read_options(reader, endian, ())?;
            result.ambient_occlusion_channel = u8::read_options(reader, endian, ())?;
            reader.seek(std::io::SeekFrom::Current(2))?;
        }

        Ok(result)
    }
}

impl BinWrite for LambertAttributes {
    type Args<'a> = (&'a LambertVersion,);

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        if matches!(args.0, LambertVersion::V4) {
            self.vertex_info.write_options(writer, endian, (false,))?;
        }

        if args.0 != &LambertVersion::V0 {
            self.flags.write_options(writer, endian, ())?;
        } else {
            (self.flags & !LambertFlags::USE_DYNAMIC_LIGHTS).write_options(writer, endian, ())?;
        }

        if args.0 == &LambertVersion::V3 {
            self.vertex_info.write_options(writer, endian, (false,))?;
        }

        if args.0 == &LambertVersion::V4 {
            self.texture_channel.write_options(writer, endian, ())?;
            (self.ambient_occlusion_channel).write_options(writer, endian, ())?;
            writer.write_all(&[0u8; 2])?;
        }

        Ok(())
    }
}

#[binrw]
#[derive(Clone, Debug, Default)]
pub struct LambertRenderBlock {
    pub version: LambertVersion,
    #[brw(args(&version))]
    pub attributes: LambertAttributes,
    #[brw(args(attributes.vertex_info.format))]
    pub vertices: Vertices<GeneralVertex>,
    #[brw(args(vertices.len()))]
    pub indices: Indices<u16>,
}

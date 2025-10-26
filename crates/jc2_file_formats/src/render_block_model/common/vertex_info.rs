use binrw::{BinRead, BinWrite, binrw};

use crate::math::{Vec2, Vec4};

#[binrw]
#[brw(repr = u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum VertexFormat {
    #[default]
    F32,
    I16,
}

#[derive(Clone, Debug)]
pub struct VertexInfo {
    pub format: VertexFormat,
    pub scale: f32,
    pub uv0_extent: Vec2<f32>,
    pub uv1_extent: Vec2<f32>,
    pub color_extent: f32,
    pub color: Vec4<u8>,
}

impl Default for VertexInfo {
    fn default() -> Self {
        Self {
            format: Default::default(),
            scale: 1.0,
            uv0_extent: Vec2::splat(1.0),
            uv1_extent: Vec2::splat(1.0),
            color_extent: 1.0,
            color: Vec4::splat(255),
        }
    }
}

impl BinRead for VertexInfo {
    type Args<'a> = (bool,);

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let mut result = Self {
            format: VertexFormat::read_options(reader, endian, ())?,
            scale: f32::read_options(reader, endian, ())?,
            uv0_extent: Vec2::<f32>::read_options(reader, endian, ())?,
            ..Default::default()
        };
        if args.0 {
            result.uv1_extent = Vec2::<f32>::read_options(reader, endian, ())?;
        } else {
            result.uv1_extent = Vec2::splat(result.uv0_extent.y);
            result.uv0_extent = Vec2::splat(result.uv0_extent.x);
        }
        result.color_extent = f32::read_options(reader, endian, ())?;
        result.color = Vec4::<u8>::read_options(reader, endian, ())?;
        Ok(result)
    }
}

impl BinWrite for VertexInfo {
    type Args<'a> = (bool,);

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        self.format.write_options(writer, endian, ())?;
        self.scale.write_options(writer, endian, ())?;
        if args.0 {
            self.uv1_extent.write_options(writer, endian, ())?;
            self.uv1_extent.write_options(writer, endian, ())?;
        } else {
            self.uv0_extent.x.write_options(writer, endian, ())?;
            self.uv1_extent.x.write_options(writer, endian, ())?;
        }
        self.color_extent.write_options(writer, endian, ())?;
        self.color.write_options(writer, endian, ())?;
        Ok(())
    }
}

use std::io::{Read, Seek, Write};

use binrw::{BinRead, BinWrite, binrw};
use thiserror::Error;

use crate::math::Vec3;

mod render_block;
pub use render_block::*;

mod common;
pub use common::*;

mod vertex_format;
pub use vertex_format::*;

#[binrw]
#[derive(Clone, Debug)]
pub struct RenderBlockModel {
    pub endian: RenderBlockModelEndian,
    #[brw(magic = b"RBMDL")]
    #[brw(assert(version.x == 1), assert(version.y == 13))]
    #[brw(is_little(matches!(endian, RenderBlockModelEndian::Little)))]
    pub version: Vec3<u32>,
    #[brw(is_little(matches!(endian, RenderBlockModelEndian::Little)))]
    pub min: Vec3<f32>,
    #[brw(is_little(matches!(endian, RenderBlockModelEndian::Little)))]
    pub max: Vec3<f32>,
    #[brw(is_little(matches!(endian, RenderBlockModelEndian::Little)))]
    pub blocks: RenderBlocks,
}

impl RenderBlockModel {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, binrw::Error> {
        #[cfg(target_endian = "little")]
        return Self::read_le(reader);

        #[cfg(target_endian = "big")]
        return Self::read_be(reader);
    }

    pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<(), binrw::Error> {
        #[cfg(target_endian = "little")]
        return self.write_le(writer);

        #[cfg(target_endian = "big")]
        return self.write_be(writer);
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderBlockModelEndian {
    #[brw(magic = b"\x05\x00\x00\x00")]
    Little,
    #[brw(magic = b"\x00\x00\x00\x05")]
    Big,
}

#[derive(Error, Debug)]
pub enum RenderBlockError {
    #[error("invalid vertex format (expected {expected:?}, found {found:?}")]
    InvalidVertexFormat {
        expected: VertexFormat,
        found: VertexFormat,
    },
    #[error("invalid length")]
    InvalidArrayLength,
    #[error("invalid block footer")]
    InvalidBlockFooter,
}

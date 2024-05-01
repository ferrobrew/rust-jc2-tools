use binrw::{binrw, BinRead, BinWrite};
use bitflags::bitflags;

use crate::render_block_model::{
    DeformTable, DeformableVertex, IndexBuffer, Material, VertexBuffer,
};

#[binrw]
#[brw(repr = u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum DeformableWindowVersion {
    V0,
    #[default]
    V1,
    V2,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
    pub struct DeformableWindowFlags: u32 {
        const DARK_WINDOW = 1 << 0;
    }
}

#[binrw]
#[derive(Clone, Debug, Default)]
pub struct DeformableWindowAttributes {
    pub flags: DeformableWindowFlags,
}

#[derive(Clone, Debug, Default)]
pub struct DeformableWindowRenderBlock {
    pub version: DeformableWindowVersion,
    pub attributes: DeformableWindowAttributes,
    pub material: Material,
    pub vertices: VertexBuffer<DeformableVertex>,
    pub indices: IndexBuffer<u16>,
    pub deform_table: DeformTable,
}

impl BinRead for DeformableWindowRenderBlock {
    type Args<'a> = ();

    #[inline]
    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let mut result = Self {
            version: DeformableWindowVersion::read_options(reader, endian, ())?,
            ..Default::default()
        };
        if (result.version as u8) < 2 {
            result.material = Material::read_options(reader, endian, ())?;
            result.vertices = VertexBuffer::<DeformableVertex>::read_options(reader, endian, ())?;
            result.indices =
                IndexBuffer::<u16>::read_options(reader, endian, (result.vertices.len(),))?;
            result.deform_table = DeformTable::read_options(reader, endian, ())?;
            if result.version == DeformableWindowVersion::V1 {
                result.attributes = DeformableWindowAttributes::read_options(reader, endian, ())?;
            }
        } else {
            result.attributes = DeformableWindowAttributes::read_options(reader, endian, ())?;
            result.deform_table = DeformTable::read_options(reader, endian, ())?;
            result.material = Material::read_options(reader, endian, ())?;
            result.vertices = VertexBuffer::<DeformableVertex>::read_options(reader, endian, ())?;
            result.indices =
                IndexBuffer::<u16>::read_options(reader, endian, (result.vertices.len(),))?;
        }
        Ok(result)
    }
}

impl BinWrite for DeformableWindowRenderBlock {
    type Args<'a> = ();

    #[inline]
    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        self.version.write_options(writer, endian, ())?;
        if (self.version as u8) < 2 {
            self.material.write_options(writer, endian, ())?;
            self.vertices.write_options(writer, endian, ())?;
            self.indices
                .write_options(writer, endian, (self.vertices.len(),))?;
            self.deform_table.write_options(writer, endian, ())?;
            if self.version == DeformableWindowVersion::V1 {
                self.attributes.write_options(writer, endian, ())?;
            }
        } else {
            self.attributes.write_options(writer, endian, ())?;
            self.deform_table.write_options(writer, endian, ())?;
            self.material.write_options(writer, endian, ())?;
            self.vertices.write_options(writer, endian, ())?;
            self.indices
                .write_options(writer, endian, (self.vertices.len(),))?;
        }
        Ok(())
    }
}

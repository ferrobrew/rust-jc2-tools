use binrw::{BinRead, BinWrite};

use crate::render_block_model::{RenderBlockError, Vertex};

#[derive(Clone, Debug)]
pub struct SkinBatch {
    pub size: u32,
    pub offset: u32,
    pub bone_indices: Vec<u16>, // TODO: size depends on platform...
}

impl Vertex for SkinBatch {
    type VertexArgs = ();
}

impl BinRead for SkinBatch {
    type Args<'a> = ();

    fn read_options<R: std::io::prelude::Read + std::io::prelude::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<Self> {
        let size = u32::read_options(reader, endian, ())?;
        let offset = u32::read_options(reader, endian, ())?;
        let length = u32::read_options(reader, endian, ())?;
        let mut bone_indices = Vec::with_capacity(length as usize);
        for _ in 0..length {
            bone_indices.push(u16::read_options(reader, endian, ())?);
        }
        Ok(Self {
            size,
            offset,
            bone_indices,
        })
    }
}

impl BinWrite for SkinBatch {
    type Args<'a> = ();

    fn write_options<W: std::io::prelude::Write + std::io::prelude::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        type BinError = binrw::Error;

        if let Ok(length) = u32::try_from(self.bone_indices.len()) {
            self.size.write_options(writer, endian, ())?;
            self.offset.write_options(writer, endian, ())?;
            length.write_options(writer, endian, ())?;
            for vertex in &self.bone_indices {
                vertex.write_options(writer, endian, args)?;
            }
            Ok(())
        } else {
            Err(BinError::Custom {
                pos: writer.stream_position()?,
                err: Box::new(RenderBlockError::InvalidArrayLength),
            })
        }
    }
}

impl Default for SkinBatch {
    fn default() -> Self {
        Self {
            size: 0u32,
            offset: 0u32,
            bone_indices: vec![0u16; 18],
        }
    }
}

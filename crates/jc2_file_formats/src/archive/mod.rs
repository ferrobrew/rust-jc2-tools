use std::{
    collections::HashMap,
    io::{Read, Seek, Write},
};

use binrw::{binrw, parser, writer, BinRead, BinResult, BinWrite};
use jc2_hashing::HashString;

#[binrw]
#[derive(Clone, Debug)]
pub struct ArchiveTableEntry {
    pub offset: u32,
    pub size: u32,
}

#[binrw]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    #[brw(magic = b"\x00\x08\x00\x00")]
    Little,
    #[brw(magic = b"\x00\x00\x08\x00")]
    Big,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ArchiveTable {
    pub endian: Endian,
    #[brw(is_little(matches!(endian, Endian::Little)))]
    #[br(parse_with = Self::parse_entries)]
    #[bw(write_with = Self::write_entries)]
    pub entries: HashMap<HashString, ArchiveTableEntry>,
}

impl ArchiveTable {
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

    #[parser(reader, endian)]
    fn parse_entries() -> BinResult<HashMap<HashString, ArchiveTableEntry>> {
        let stream_position = reader.stream_position()?;
        let stream_length = reader.seek(std::io::SeekFrom::End(0))?;
        let stream_remaining = stream_length - stream_position;
        let count = stream_remaining / 12u64;

        reader.seek(std::io::SeekFrom::Start(stream_position))?;

        let mut result = HashMap::with_capacity(count as usize);
        for _ in 0..count {
            result.insert(
                HashString::read_options(reader, endian, ())?,
                ArchiveTableEntry::read_options(reader, endian, ())?,
            );
        }
        Ok(result)
    }

    #[writer(writer, endian)]
    fn write_entries(entries: &HashMap<HashString, ArchiveTableEntry>) -> BinResult<()> {
        for (hash, entry) in entries {
            hash.write_options(writer, endian, ())?;
            entry.write_options(writer, endian, ())?;
        }
        Ok(())
    }
}

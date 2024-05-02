use std::{
    collections::HashMap,
    io::{Read, Seek, Write},
};

use binrw::{binrw, parser, writer, BinRead, BinResult, BinWrite};
use jc2_hashing::HashString;

use crate::string::LengthString;

#[binrw]
#[repr(C)]
#[derive(Clone, Debug)]
pub struct ArchiveTableEntry {
    pub offset: u32,
    pub size: u32,
}

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ArchiveEndian {
    #[default]
    #[brw(magic = b"\x00\x08\x00\x00")]
    Little,
    #[brw(magic = b"\x00\x00\x08\x00")]
    Big,
}

#[binrw]
#[derive(Clone, Debug)]
pub struct ArchiveTable {
    pub endian: ArchiveEndian,
    #[br(parse_with = Self::parse_entries)]
    #[bw(write_with = Self::write_entries)]
    #[brw(is_little(matches!(endian, ArchiveEndian::Little)))]
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

#[binrw]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum StreamArchiveEndian {
    #[default]
    #[brw(magic = b"\x04\x00\x00\x00SARC")]
    Little,
    #[brw(magic = b"\x00\x00\x00\x04SARC")]
    Big,
}

#[binrw]
#[brw(repr = u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum StreamArchiveVersion {
    #[default]
    V2 = 2,
}

#[binrw]
#[derive(Clone, Debug, Default)]
pub struct StreamArchive {
    pub endian: StreamArchiveEndian,
    #[brw(is_little(matches!(endian, StreamArchiveEndian::Little)))]
    pub version: StreamArchiveVersion,
    #[br(parse_with = Self::parse_entries)]
    #[bw(write_with = Self::write_entries)]
    #[brw(is_little(matches!(endian, StreamArchiveEndian::Little)))]
    pub entries: HashMap<String, Vec<u8>>,
}

impl StreamArchive {
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
    fn parse_entries() -> BinResult<HashMap<String, Vec<u8>>> {
        let table_size = u32::read_options(reader, endian, ())?;
        let table_end = reader.stream_position()? + table_size as u64;

        let mut result = HashMap::with_capacity(16);
        loop {
            // We read until the table can not contain at least one empty entry
            if (table_end - reader.stream_position()?) < 16 {
                break;
            }

            // We read the name and entry
            let name = LengthString::<u32>::read_options(reader, endian, ())?.into();
            let entry = ArchiveTableEntry::read_options(reader, endian, ())?;

            // Then the data via seeking to it, then returning to where we began
            let stream_position = reader.stream_position()?;
            let mut data = vec![0u8; entry.size as usize];
            reader.seek(std::io::SeekFrom::Start(entry.offset as u64))?;
            reader.read_exact(&mut data)?;
            reader.seek(std::io::SeekFrom::Start(stream_position))?;
            result.insert(name, data);
        }

        Ok(result)
    }

    #[writer(writer, endian)]
    fn write_entries(entries: &HashMap<String, Vec<u8>>) -> BinResult<()> {
        // Small helper to align to next power of two
        #[inline(always)]
        const fn align(value: usize, alignment: usize) -> usize {
            let align = alignment - 1;
            (value + align) & !align
        }

        // Calculate the total size necessary to store all files
        let mut entry_padding: Vec<usize> = Vec::with_capacity(entries.len());
        let buffer_size = entries.values().fold(0usize, |size, entry| {
            let entry_size = entry.len();
            let aligned_entry_size = align(entry_size, 4);
            entry_padding.push(aligned_entry_size - entry_size);
            size + aligned_entry_size
        });

        // Build the final buffer, and our table of contents
        let mut table = Vec::with_capacity(entries.len());
        let mut buffer = Vec::with_capacity(buffer_size);
        for ((name, data), padding) in entries.iter().zip(&entry_padding) {
            table.push((
                LengthString::<u32>::from(name.clone()),
                ArchiveTableEntry {
                    offset: buffer.len() as u32,
                    size: data.len() as u32,
                },
            ));
            buffer.extend_from_slice(data);
            buffer.extend_from_slice(&vec![0u8; *padding]);
        }

        // Write the table of contents size
        let (table_size, table_padding) = {
            let size = table.iter().fold(0usize, |size, entry| {
                size + entry.0.size() + std::mem::size_of_val(&entry.1)
            });
            let padding = align(size, 16) - size;
            ((size + padding) as u32, padding)
        };
        table_size.write_options(writer, endian, ())?;

        // Write the table of contents
        let table_position = writer.stream_position()? as u32;
        for (name, entry) in &mut table {
            entry.offset += table_position + table_size;
            name.write_options(writer, endian, ())?;
            entry.write_options(writer, endian, ())?;
        }
        writer.write_all(&vec![0u8; table_padding])?;

        // Finally write the data buffer
        writer.write_all(&buffer)?;

        Ok(())
    }
}

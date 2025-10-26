use std::ops::{Deref, DerefMut};

use binrw::{BinRead, BinWrite, binrw};
use jc2_hashing::HashString;

use crate::{
    BinReadWrite,
    common::{LengthVec, NullString},
    math::{Vec2, Vec3, Vec4},
};

use super::PropertyValue;

#[binrw]
#[brw(magic = b"PCBB")]
#[derive(Clone, Default, Debug)]
pub struct PropertyBlockFile(
    #[br(parse_with = Self::parse)]
    #[bw(write_with = Self::write)]
    pub PropertyBlockContainer,
);

impl PropertyBlockFile {
    const DATA_OFFSET: u64 = 8;

    #[inline]
    #[binrw::parser(reader, endian)]
    fn parse() -> binrw::BinResult<PropertyBlockContainer> {
        reader.seek_relative(4)?;
        Ok(PropertyBlockContainer::read_options(reader, endian, ())?)
    }

    #[inline]
    #[binrw::writer(writer, endian)]
    fn write(value: &PropertyBlockContainer) -> binrw::BinResult<()> {
        // Write an initial size of zero
        let start = writer.stream_position()?;
        0u32.write_options(writer, endian, ())?;
        value.write_options(writer, endian, ())?;

        // Seek back to initial size
        let end = writer.stream_position()?;
        writer.seek(std::io::SeekFrom::Start(start))?;

        // Overwrite it with the final calcuated size
        let size = (end - start - 4) as u32;
        size.write_options(writer, endian, ())?;
        writer.seek_relative(size as i64)?;

        Ok(())
    }
}

#[inline]
fn offset_to_position(offset: u32) -> u64 {
    offset as u64 + PropertyBlockFile::DATA_OFFSET
}

#[inline]
fn position_to_offset(position: u64) -> u32 {
    (position - PropertyBlockFile::DATA_OFFSET) as u32
}

#[binrw]
#[derive(Clone, Default, Debug, PartialEq)]
pub struct PropertyBlockContainer(
    #[br(parse_with = Self::parse)]
    #[bw(write_with = Self::write)]
    pub Vec<PropertyBlockNode>,
);

impl PropertyBlockContainer {
    #[inline]
    #[binrw::parser(reader, endian)]
    fn parse() -> binrw::BinResult<Vec<PropertyBlockNode>> {
        let mut result = vec![];
        loop {
            result.push(PropertyBlockNode::read_options(reader, endian, ())?);

            // Continue reading until we hit an empty `PropertyBlockPointer`
            let next = u32::read_options(reader, endian, ())?;
            if next != u32::MAX {
                reader.seek(std::io::SeekFrom::Start(offset_to_position(next)))?;
            } else {
                break;
            }
        }
        Ok(result)
    }

    #[inline]
    #[binrw::writer(writer, endian)]
    fn write(value: &Vec<PropertyBlockNode>) -> binrw::BinResult<()> {
        let mut it = value.iter().peekable();
        while let Some(node) = it.next() {
            let start = writer.stream_position()?;
            node.write_options(writer, endian, ())?;

            // Patch `PropertyBlockPointer` to point to next node
            if it.peek().is_some() {
                let end = writer.stream_position()?;
                writer.seek(std::io::SeekFrom::Start(start + 12))?;
                position_to_offset(end).write_options(writer, endian, ())?;
                writer.seek(std::io::SeekFrom::Start(end))?;
            }
        }
        Ok(())
    }
}

impl Deref for PropertyBlockContainer {
    type Target = Vec<PropertyBlockNode>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PropertyBlockContainer {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[PropertyBlockNode]> for PropertyBlockContainer {
    #[inline]
    fn as_ref(&self) -> &[PropertyBlockNode] {
        &self.0
    }
}

impl AsMut<[PropertyBlockNode]> for PropertyBlockContainer {
    #[inline]
    fn as_mut(&mut self) -> &mut [PropertyBlockNode] {
        &mut self.0
    }
}

impl From<PropertyBlockContainer> for Vec<PropertyBlockNode> {
    #[inline]
    fn from(value: PropertyBlockContainer) -> Self {
        value.0
    }
}

impl From<Vec<PropertyBlockNode>> for PropertyBlockContainer {
    #[inline]
    fn from(value: Vec<PropertyBlockNode>) -> Self {
        Self(value)
    }
}

impl From<&[PropertyBlockNode]> for PropertyBlockContainer {
    #[inline]
    fn from(value: &[PropertyBlockNode]) -> Self {
        Self(value.into())
    }
}

#[binrw]
#[derive(Clone, Default, Debug, PartialEq)]
pub struct PropertyBlockNode {
    pub hash: HashString,
    pub value: PropertyBlockNodeValue,
}

#[binrw]
#[derive(Clone, Default, Debug, PartialEq)]
pub enum PropertyBlockNodeValue {
    #[default]
    #[brw(magic = 0u32)]
    Empty,
    #[brw(magic = 1u16)]
    Container(
        #[br(parse_with = Self::parse_container)]
        #[bw(write_with = Self::write_container)]
        PropertyBlockPointer<PropertyBlockContainer>,
    ),
    #[brw(magic = 2u16)]
    Value(
        #[br(parse_with = Self::parse_value)]
        #[bw(write_with = Self::write_value)]
        PropertyBlockValue,
    ),
}

impl PropertyBlockNodeValue {
    #[inline]
    #[binrw::parser(reader, endian)]
    fn parse<T: Default + PartialEq + BinReadWrite>() -> binrw::BinResult<T> {
        reader.seek_relative(2)?;
        Ok(PropertyBlockPointer::<T>::read_options(reader, endian, ())?.0)
    }

    #[inline]
    #[binrw::writer(writer, endian)]
    fn write<T: Default + PartialEq + BinReadWrite>(value: &T) -> binrw::BinResult<()> {
        0u16.write_options(writer, endian, ())?;

        // Manually write two `PropertyBlockPointer` instances, leaving one empty until the second pass
        if value != &T::default() {
            position_to_offset(writer.stream_position()? + 8).write_options(writer, endian, ())?;
            u32::MAX.write_options(writer, endian, ())?;
            value.write_options(writer, endian, ())?;
        } else {
            u32::MAX.write_options(writer, endian, ())?;
            u32::MAX.write_options(writer, endian, ())?;
        }

        Ok(())
    }

    #[inline]
    #[binrw::parser(reader, endian)]
    fn parse_container() -> binrw::BinResult<PropertyBlockPointer<PropertyBlockContainer>> {
        Self::parse(reader, endian, ())
    }

    #[inline]
    #[binrw::writer(writer, endian)]
    fn write_container(
        value: &PropertyBlockPointer<PropertyBlockContainer>,
    ) -> binrw::BinResult<()> {
        Self::write(value, writer, endian, ())
    }

    #[inline]
    #[binrw::parser(reader, endian)]
    fn parse_value() -> binrw::BinResult<PropertyBlockValue> {
        Self::parse(reader, endian, ())
    }

    #[inline]
    #[binrw::writer(writer, endian)]
    fn write_value(value: &PropertyBlockValue) -> binrw::BinResult<()> {
        Self::write(value, writer, endian, ())
    }
}

impl From<PropertyBlockContainer> for PropertyBlockNodeValue {
    fn from(value: PropertyBlockContainer) -> Self {
        PropertyBlockNodeValue::Container(value.into())
    }
}

impl<T: Into<PropertyBlockValue>> From<T> for PropertyBlockNodeValue {
    fn from(value: T) -> Self {
        PropertyBlockNodeValue::Value(value.into())
    }
}

#[binrw]
#[derive(Clone, Default, Debug, PartialEq)]
pub enum PropertyBlockValue {
    #[default]
    #[brw(magic = 0u32)]
    Empty,
    #[brw(magic = 1u32)]
    I32(i32),
    #[brw(magic = 2u32)]
    F32(f32),
    #[brw(magic = 3u32)]
    String(PropertyBlockPointer<NullString>),
    #[brw(magic = 4u32)]
    Vec2(PropertyBlockPointer<Vec2<f32>>),
    #[brw(magic = 5u32)]
    Vec3(PropertyBlockPointer<Vec3<f32>>),
    #[brw(magic = 6u32)]
    Vec4(PropertyBlockPointer<Vec4<f32>>),
    #[brw(magic = 7u32)]
    Mat3x3(PropertyBlockPointer<[f32; 9]>),
    #[brw(magic = 8u32)]
    Mat3x4(PropertyBlockPointer<[f32; 12]>),
    #[brw(magic = 9u32)]
    VecI32(PropertyBlockPointer<LengthVec<i32, u32, true>>),
    #[brw(magic = 10u32)]
    VecF32(PropertyBlockPointer<LengthVec<f32, u32, true>>),
}

impl From<PropertyValue> for PropertyBlockValue {
    fn from(value: PropertyValue) -> Self {
        match value {
            PropertyValue::Empty => Self::Empty,
            PropertyValue::I32(value) => Self::I32(value),
            PropertyValue::F32(value) => Self::F32(value),
            PropertyValue::String(value) => Self::String(value.into()),
            PropertyValue::Vec2(value) => Self::Vec2(value.into()),
            PropertyValue::Vec3(value) => Self::Vec3(value.into()),
            PropertyValue::Vec4(value) => Self::Vec4(value.into()),
            PropertyValue::Mat3x3(value) => Self::Mat3x3(value.into()),
            PropertyValue::Mat3x4(value) => Self::Mat3x4(value.into()),
            PropertyValue::VecI32(value) => Self::VecI32(value.into()),
            PropertyValue::VecF32(value) => Self::VecF32(value.into()),
        }
    }
}

#[binrw]
#[derive(Clone, Default, Debug, PartialEq)]
pub struct PropertyBlockPointer<T: Default + PartialEq + BinReadWrite>(
    #[br(parse_with = Self::parse)]
    #[bw(write_with = Self::write)]
    pub T,
);

impl<T: Default + PartialEq + BinReadWrite> PropertyBlockPointer<T> {
    #[inline]
    #[binrw::parser(reader, endian)]
    fn parse() -> binrw::BinResult<T> {
        let offset = u32::read_options(reader, endian, ())?;
        if offset != u32::MAX {
            let position = reader.stream_position()?;
            reader.seek(std::io::SeekFrom::Start(offset_to_position(offset)))?;

            let value = T::read_options(reader, endian, ())?;
            reader.seek(std::io::SeekFrom::Start(position))?;

            Ok(value)
        } else {
            Ok(T::default())
        }
    }

    #[inline]
    #[binrw::writer(writer, endian)]
    fn write(value: &T) -> binrw::BinResult<()> {
        if value != &T::default() {
            position_to_offset(writer.stream_position()? + 4).write_options(writer, endian, ())?;
            value.write_options(writer, endian, ())?;

            // We must ensure all data is 4-byte aligned
            let align = (writer.stream_position()? % 4) as usize;
            if align != 0 {
                [0u8; 4][0..4 - align].write_options(writer, endian, ())?;
            }
        } else {
            u32::MAX.write_options(writer, endian, ())?;
        }

        Ok(())
    }
}

impl<T: Default + PartialEq + BinReadWrite> Deref for PropertyBlockPointer<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Default + PartialEq + BinReadWrite> DerefMut for PropertyBlockPointer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Default + PartialEq + BinReadWrite> AsRef<T> for PropertyBlockPointer<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: Default + PartialEq + BinReadWrite> AsMut<T> for PropertyBlockPointer<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl From<String> for PropertyBlockPointer<NullString> {
    #[inline]
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<Vec<i32>> for PropertyBlockPointer<LengthVec<i32, u32, true>> {
    #[inline]
    fn from(value: Vec<i32>) -> Self {
        Self(value.into())
    }
}

impl From<Vec<f32>> for PropertyBlockPointer<LengthVec<f32, u32, true>> {
    #[inline]
    fn from(value: Vec<f32>) -> Self {
        Self(value.into())
    }
}

impl<T: Default + PartialEq + BinReadWrite> From<T> for PropertyBlockPointer<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

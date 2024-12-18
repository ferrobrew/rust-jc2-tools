use binrw::binrw;
use jc2_hashing::HashString;

use crate::{
    common::{LengthString, LengthVec},
    math::{Vec2, Vec3, Vec4},
};

use super::PropertyValue;

#[binrw]
#[derive(Clone, Default, Debug)]
pub struct PropertyFile(pub LengthVec<PropertyFileSection, u8>);

#[binrw]
#[derive(Clone, Debug)]
pub enum PropertyFileSection {
    #[brw(magic = 0u16)]
    Empty(u16),
    #[brw(magic = 1u16)]
    Container(LengthVec<(LengthString<u32>, PropertyFile), u16>),
    #[brw(magic = 2u16)]
    Value(LengthVec<(LengthString<u32>, PropertyFileValue), u16>),
    #[brw(magic = 3u16)]
    Raw(LengthVec<u8, u16>),
    #[brw(magic = 4u16)]
    HashedContainer(LengthVec<(HashString, PropertyFile), u16>),
    #[brw(magic = 5u16)]
    HashedValue(LengthVec<(HashString, PropertyFileValue), u16>),
}

impl Default for PropertyFileSection {
    fn default() -> Self {
        Self::Empty(0)
    }
}

#[binrw]
#[derive(Clone, Default, Debug, PartialEq)]
pub enum PropertyFileValue {
    #[default]
    #[brw(magic = 0u8)]
    Empty,
    #[brw(magic = 1u8)]
    I32(i32),
    #[brw(magic = 2u8)]
    F32(f32),
    #[brw(magic = 3u8)]
    String(LengthString<u16>),
    #[brw(magic = 4u8)]
    Vec2(Vec2<f32>),
    #[brw(magic = 5u8)]
    Vec3(Vec3<f32>),
    #[brw(magic = 6u8)]
    Vec4(Vec4<f32>),
    #[brw(magic = 7u8)]
    Mat3x3([f32; 9]),
    #[brw(magic = 8u8)]
    Mat3x4([f32; 12]),
    #[brw(magic = 9u8)]
    VecI32(LengthVec<i32, u32>),
    #[brw(magic = 10u8)]
    VecF32(LengthVec<f32, u32>),
}

impl From<PropertyValue> for PropertyFileValue {
    fn from(value: PropertyValue) -> Self {
        match value {
            PropertyValue::Empty => Self::Empty,
            PropertyValue::I32(value) => Self::I32(value),
            PropertyValue::F32(value) => Self::F32(value),
            PropertyValue::String(value) => Self::String(value.into()),
            PropertyValue::Vec2(value) => Self::Vec2(value),
            PropertyValue::Vec3(value) => Self::Vec3(value),
            PropertyValue::Vec4(value) => Self::Vec4(value),
            PropertyValue::Mat3x3(value) => Self::Mat3x3(value),
            PropertyValue::Mat3x4(value) => Self::Mat3x4(value),
            PropertyValue::VecI32(value) => Self::VecI32(value.into()),
            PropertyValue::VecF32(value) => Self::VecF32(value.into()),
        }
    }
}

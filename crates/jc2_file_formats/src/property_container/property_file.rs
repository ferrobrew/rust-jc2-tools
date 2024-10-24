use binrw::binrw;
use jc2_hashing::HashString;

use crate::{
    common::{LengthString, LengthVec},
    math::{Vec2, Vec3, Vec4},
};

#[binrw]
#[derive(Clone, Default, Debug)]
pub struct PropertyFile {
    pub sections: LengthVec<PropertyFileSection, u8>,
}

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
    #[brw(magic = 11u8)]
    VecU8(LengthVec<u8, u32>),
}

impl From<i32> for PropertyFileValue {
    fn from(value: i32) -> Self {
        PropertyFileValue::I32(value)
    }
}

impl From<f32> for PropertyFileValue {
    fn from(value: f32) -> Self {
        PropertyFileValue::F32(value)
    }
}

impl From<&str> for PropertyFileValue {
    fn from(value: &str) -> Self {
        PropertyFileValue::String(value.into())
    }
}

impl From<String> for PropertyFileValue {
    fn from(value: String) -> Self {
        PropertyFileValue::String(value.into())
    }
}

impl From<[f32; 2]> for PropertyFileValue {
    fn from(value: [f32; 2]) -> Self {
        PropertyFileValue::Vec2(value.into())
    }
}

impl From<[f32; 3]> for PropertyFileValue {
    fn from(value: [f32; 3]) -> Self {
        PropertyFileValue::Vec3(value.into())
    }
}

impl From<[f32; 4]> for PropertyFileValue {
    fn from(value: [f32; 4]) -> Self {
        PropertyFileValue::Vec4(value.into())
    }
}

impl From<Vec2<f32>> for PropertyFileValue {
    fn from(value: Vec2<f32>) -> Self {
        PropertyFileValue::Vec2(value)
    }
}

impl From<Vec3<f32>> for PropertyFileValue {
    fn from(value: Vec3<f32>) -> Self {
        PropertyFileValue::Vec3(value)
    }
}

impl From<Vec4<f32>> for PropertyFileValue {
    fn from(value: Vec4<f32>) -> Self {
        PropertyFileValue::Vec4(value)
    }
}

impl From<[f32; 9]> for PropertyFileValue {
    fn from(value: [f32; 9]) -> Self {
        PropertyFileValue::Mat3x3(value)
    }
}

impl From<[f32; 12]> for PropertyFileValue {
    fn from(value: [f32; 12]) -> Self {
        PropertyFileValue::Mat3x4(value)
    }
}

impl From<&[i32]> for PropertyFileValue {
    fn from(value: &[i32]) -> Self {
        PropertyFileValue::VecI32(value.into())
    }
}

impl From<&[f32]> for PropertyFileValue {
    fn from(value: &[f32]) -> Self {
        PropertyFileValue::VecF32(value.into())
    }
}

impl From<&[u8]> for PropertyFileValue {
    fn from(value: &[u8]) -> Self {
        PropertyFileValue::VecU8(value.into())
    }
}

impl From<Vec<i32>> for PropertyFileValue {
    fn from(value: Vec<i32>) -> Self {
        PropertyFileValue::VecI32(value.into())
    }
}

impl From<Vec<f32>> for PropertyFileValue {
    fn from(value: Vec<f32>) -> Self {
        PropertyFileValue::VecF32(value.into())
    }
}

impl From<Vec<u8>> for PropertyFileValue {
    fn from(value: Vec<u8>) -> Self {
        PropertyFileValue::VecU8(value.into())
    }
}

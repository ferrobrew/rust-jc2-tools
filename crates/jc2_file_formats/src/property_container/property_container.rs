use std::collections::HashMap;

use jc2_hashing::HashString;

use crate::math::{Vec2, Vec3, Vec4};

use super::{
    PropertyBlockContainer, PropertyBlockFile, PropertyBlockNodeValue, PropertyBlockValue,
    PropertyFile, PropertyFileSection, PropertyFileValue,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyContainer(HashMap<HashString, PropertyEntry>);

impl From<PropertyBlockFile> for PropertyContainer {
    fn from(value: PropertyBlockFile) -> Self {
        value.0.into()
    }
}

impl From<PropertyBlockContainer> for PropertyContainer {
    fn from(value: PropertyBlockContainer) -> Self {
        let mut result = HashMap::<HashString, PropertyEntry>::new();
        for node in value.0.into_iter() {
            match node.value {
                PropertyBlockNodeValue::Empty => {}
                PropertyBlockNodeValue::Container(container) => {
                    result.insert(node.hash, PropertyContainer::from(container.0).into());
                }
                PropertyBlockNodeValue::Value(value) => {
                    if value == PropertyBlockValue::Empty {
                        continue;
                    }
                    let value: PropertyValue = match value {
                        PropertyBlockValue::Empty => unreachable!(),
                        PropertyBlockValue::I32(value) => value.into(),
                        PropertyBlockValue::F32(value) => value.into(),
                        PropertyBlockValue::String(value) => value.0.value.into(),
                        PropertyBlockValue::Vec2(value) => value.0.into(),
                        PropertyBlockValue::Vec3(value) => value.0.into(),
                        PropertyBlockValue::Vec4(value) => value.0.into(),
                        PropertyBlockValue::Mat3x3(value) => value.0.into(),
                        PropertyBlockValue::Mat3x4(value) => value.0.into(),
                        PropertyBlockValue::VecI32(value) => value.0.value.into(),
                        PropertyBlockValue::VecF32(value) => value.0.value.into(),
                    };
                    result.insert(node.hash, value.into());
                }
            }
        }
        Self(result)
    }
}

impl From<PropertyFile> for PropertyContainer {
    fn from(value: PropertyFile) -> Self {
        let mut result = HashMap::<HashString, PropertyEntry>::new();
        for section in value.0.into_iter() {
            let to_value = |value| -> PropertyValue {
                match value {
                    PropertyFileValue::Empty => unreachable!(),
                    PropertyFileValue::I32(value) => value.into(),
                    PropertyFileValue::F32(value) => value.into(),
                    PropertyFileValue::String(value) => value.value.into(),
                    PropertyFileValue::Vec2(value) => value.into(),
                    PropertyFileValue::Vec3(value) => value.into(),
                    PropertyFileValue::Vec4(value) => value.into(),
                    PropertyFileValue::Mat3x3(value) => value.into(),
                    PropertyFileValue::Mat3x4(value) => value.into(),
                    PropertyFileValue::VecI32(value) => value.value.into(),
                    PropertyFileValue::VecF32(value) => value.value.into(),
                }
            };

            match section {
                PropertyFileSection::Container(containers) => {
                    for (key, container) in containers.into_iter() {
                        result.insert(key.value.into(), PropertyContainer::from(container).into());
                    }
                }
                PropertyFileSection::Value(values) => {
                    for (key, value) in values.into_iter() {
                        if value == PropertyFileValue::Empty {
                            continue;
                        }
                        result.insert(key.value.into(), to_value(value).into());
                    }
                }
                PropertyFileSection::HashedContainer(containers) => {
                    for (key, container) in containers.into_iter() {
                        result.insert(key, PropertyContainer::from(container).into());
                    }
                }
                PropertyFileSection::HashedValue(values) => {
                    for (key, value) in values.into_iter() {
                        if value == PropertyFileValue::Empty {
                            continue;
                        }
                        result.insert(key, to_value(value).into());
                    }
                }
                _ => {}
            }
        }
        Self(result)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyEntry {
    Container(PropertyContainer),
    Value(PropertyValue),
}

impl From<PropertyContainer> for PropertyEntry {
    fn from(value: PropertyContainer) -> Self {
        PropertyEntry::Container(value)
    }
}

impl<T: Into<PropertyValue>> From<T> for PropertyEntry {
    fn from(value: T) -> Self {
        PropertyEntry::Value(value.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    I32(i32),
    F32(f32),
    String(String),
    Vec2(Vec2<f32>),
    Vec3(Vec3<f32>),
    Vec4(Vec4<f32>),
    Mat3x3([f32; 9]),
    Mat3x4([f32; 12]),
    VecI32(Vec<i32>),
    VecF32(Vec<f32>),
}

impl From<i32> for PropertyValue {
    fn from(value: i32) -> Self {
        PropertyValue::I32(value)
    }
}

impl From<f32> for PropertyValue {
    fn from(value: f32) -> Self {
        PropertyValue::F32(value)
    }
}

impl From<&str> for PropertyValue {
    fn from(value: &str) -> Self {
        PropertyValue::String(value.into())
    }
}

impl From<String> for PropertyValue {
    fn from(value: String) -> Self {
        PropertyValue::String(value)
    }
}

impl From<[f32; 2]> for PropertyValue {
    fn from(value: [f32; 2]) -> Self {
        PropertyValue::Vec2(value.into())
    }
}

impl From<[f32; 3]> for PropertyValue {
    fn from(value: [f32; 3]) -> Self {
        PropertyValue::Vec3(value.into())
    }
}

impl From<[f32; 4]> for PropertyValue {
    fn from(value: [f32; 4]) -> Self {
        PropertyValue::Vec4(value.into())
    }
}

impl From<Vec2<f32>> for PropertyValue {
    fn from(value: Vec2<f32>) -> Self {
        PropertyValue::Vec2(value)
    }
}

impl From<Vec3<f32>> for PropertyValue {
    fn from(value: Vec3<f32>) -> Self {
        PropertyValue::Vec3(value)
    }
}

impl From<Vec4<f32>> for PropertyValue {
    fn from(value: Vec4<f32>) -> Self {
        PropertyValue::Vec4(value)
    }
}

impl From<[f32; 9]> for PropertyValue {
    fn from(value: [f32; 9]) -> Self {
        PropertyValue::Mat3x3(value)
    }
}

impl From<[f32; 12]> for PropertyValue {
    fn from(value: [f32; 12]) -> Self {
        PropertyValue::Mat3x4(value)
    }
}

impl From<&[i32]> for PropertyValue {
    fn from(value: &[i32]) -> Self {
        PropertyValue::VecI32(value.into())
    }
}

impl From<&[f32]> for PropertyValue {
    fn from(value: &[f32]) -> Self {
        PropertyValue::VecF32(value.into())
    }
}

impl From<Vec<i32>> for PropertyValue {
    fn from(value: Vec<i32>) -> Self {
        PropertyValue::VecI32(value)
    }
}

impl From<Vec<f32>> for PropertyValue {
    fn from(value: Vec<f32>) -> Self {
        PropertyValue::VecF32(value)
    }
}

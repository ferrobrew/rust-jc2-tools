use std::collections::HashMap;

use jc2_hashing::HashString;

use crate::math::{Vec2, Vec3, Vec4};

use super::{
    PropertyBlockContainer, PropertyBlockFile, PropertyBlockNodeValue, PropertyBlockValue,
    PropertyFile, PropertyFileSection, PropertyFileValue,
};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct PropertyContainer(HashMap<HashString, PropertyEntry>);

impl PropertyContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, hash: impl Into<HashString>, value: impl Into<PropertyEntry>) {
        self.0.insert(hash.into(), value.into());
    }

    pub fn get(&self, hash: impl Into<HashString>) -> Option<&PropertyEntry> {
        self.0.get(&hash.into())
    }

    pub fn insert_container(
        &mut self,
        hash: impl Into<HashString>,
        value: impl Into<PropertyContainer>,
    ) {
        self.0.insert(hash.into(), value.into().into());
    }

    pub fn get_container(&self, hash: impl Into<HashString>) -> Option<&PropertyContainer> {
        self.0.get(&hash.into()).and_then(|entry| match entry {
            PropertyEntry::Container(container) => Some(container),
            _ => None,
        })
    }

    pub fn containers<'a>(&'a self) -> FilterPropertyContainerValues<'a, &PropertyContainer> {
        self.0.values().filter_map(|entry| match entry {
            PropertyEntry::Container(container) => Some(container),
            _ => None,
        })
    }

    pub fn keyed_containers<'a>(&'a self) -> FilterPropertyContainerIter<'a, &PropertyContainer> {
        self.0.iter().filter_map(|entry| match entry.1 {
            PropertyEntry::Container(container) => Some((entry.0, container)),
            _ => None,
        })
    }

    pub fn insert_value(&mut self, hash: impl Into<HashString>, value: impl Into<PropertyValue>) {
        self.0.insert(hash.into(), value.into().into());
    }

    pub fn get_value<'a, T: FromPropertyValue<'a>>(
        &'a self,
        hash: impl Into<HashString>,
    ) -> Option<T> {
        self.0.get(&hash.into()).and_then(|entry| match entry {
            PropertyEntry::Value(value) => T::from_property_value(value),
            _ => None,
        })
    }

    pub fn values<'a>(&'a self) -> FilterPropertyContainerValues<'a, &PropertyValue> {
        self.0.values().filter_map(|entry| match entry {
            PropertyEntry::Value(value) => Some(value),
            _ => None,
        })
    }

    pub fn values_filtered<'a, T: FromPropertyValue<'a>>(
        &'a self,
    ) -> FilterPropertyContainerValues<'a, T> {
        self.0.values().filter_map(|entry| match entry {
            PropertyEntry::Value(value) => T::from_property_value(value),
            _ => None,
        })
    }

    pub fn keyed_values<'a>(&'a self) -> FilterPropertyContainerIter<'a, &PropertyValue> {
        self.0.iter().filter_map(|entry| match entry.1 {
            PropertyEntry::Value(value) => Some((entry.0, value)),
            _ => None,
        })
    }

    pub fn keyed_valued_filtered<'a, T: FromPropertyValue<'a>>(
        &'a self,
    ) -> FilterPropertyContainerIter<'a, T> {
        self.0.iter().filter_map(|entry| match entry.1 {
            PropertyEntry::Value(value) => T::from_property_value(value).map(|t| (entry.0, t)),
            _ => None,
        })
    }
}

type FilterMapValues<'a, K, V, R> =
    std::iter::FilterMap<std::collections::hash_map::Values<'a, K, V>, fn(&'a V) -> Option<R>>;
type FilterPropertyContainerValues<'a, T> = FilterMapValues<'a, HashString, PropertyEntry, T>;

type FilterMapIter<'a, K, V, R> = std::iter::FilterMap<
    std::collections::hash_map::Iter<'a, K, V>,
    fn((&'a K, &'a V)) -> Option<(&'a K, R)>,
>;
type FilterPropertyContainerIter<'a, T> = FilterMapIter<'a, HashString, PropertyEntry, T>;

impl<T: Sized + Into<HashMap<HashString, PropertyEntry>>> From<T> for PropertyContainer {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

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
            match section {
                PropertyFileSection::Container(containers) => {
                    for (key, container) in containers.into_iter() {
                        result.insert(key.value.into(), PropertyContainer::from(container).into());
                    }
                }
                PropertyFileSection::Value(values) => {
                    for (key, value) in values.into_iter() {
                        result.insert(key.value.into(), value.into());
                    }
                }
                PropertyFileSection::HashedContainer(containers) => {
                    for (key, container) in containers.into_iter() {
                        result.insert(key, PropertyContainer::from(container).into());
                    }
                }
                PropertyFileSection::HashedValue(values) => {
                    for (key, value) in values.into_iter() {
                        result.insert(key, value.into());
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
    Empty,
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

impl From<PropertyBlockValue> for PropertyValue {
    fn from(value: PropertyBlockValue) -> Self {
        match value {
            PropertyBlockValue::Empty => Self::Empty,
            PropertyBlockValue::I32(value) => Self::I32(value),
            PropertyBlockValue::F32(value) => Self::F32(value),
            PropertyBlockValue::String(value) => Self::String(value.0.value),
            PropertyBlockValue::Vec2(value) => Self::Vec2(value.0),
            PropertyBlockValue::Vec3(value) => Self::Vec3(value.0),
            PropertyBlockValue::Vec4(value) => Self::Vec4(value.0),
            PropertyBlockValue::Mat3x3(value) => Self::Mat3x3(value.0),
            PropertyBlockValue::Mat3x4(value) => Self::Mat3x4(value.0),
            PropertyBlockValue::VecI32(value) => Self::VecI32(value.0.value),
            PropertyBlockValue::VecF32(value) => Self::VecF32(value.0.value),
        }
    }
}

impl From<PropertyFileValue> for PropertyValue {
    fn from(value: PropertyFileValue) -> Self {
        match value {
            PropertyFileValue::Empty => Self::Empty,
            PropertyFileValue::I32(value) => Self::I32(value),
            PropertyFileValue::F32(value) => Self::F32(value),
            PropertyFileValue::String(value) => Self::String(value.value),
            PropertyFileValue::Vec2(value) => Self::Vec2(value),
            PropertyFileValue::Vec3(value) => Self::Vec3(value),
            PropertyFileValue::Vec4(value) => Self::Vec4(value),
            PropertyFileValue::Mat3x3(value) => Self::Mat3x3(value),
            PropertyFileValue::Mat3x4(value) => Self::Mat3x4(value),
            PropertyFileValue::VecI32(value) => Self::VecI32(value.value),
            PropertyFileValue::VecF32(value) => Self::VecF32(value.value),
        }
    }
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

pub trait FromPropertyValue<'a>
where
    Self: Sized,
{
    fn from_property_value(value: &'a PropertyValue) -> Option<Self>;
}

impl FromPropertyValue<'_> for PropertyValue {
    fn from_property_value(value: &'_ PropertyValue) -> Option<Self> {
        Some(value.clone())
    }
}

impl FromPropertyValue<'_> for i32 {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::I32(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for f32 {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::F32(value) => Some(*value),
            _ => None,
        }
    }
}

impl<'a> FromPropertyValue<'a> for &'a str {
    fn from_property_value(value: &'a PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::String(value) => Some(value),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for String {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::String(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for [f32; 2] {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::Vec2(value) => Some((*value).into()),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for [f32; 3] {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::Vec3(value) => Some((*value).into()),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for [f32; 4] {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::Vec4(value) => Some((*value).into()),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for Vec2<f32> {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::Vec2(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for Vec3<f32> {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::Vec3(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for Vec4<f32> {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::Vec4(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for [f32; 9] {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::Mat3x3(value) => Some(*value),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for [f32; 12] {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::Mat3x4(value) => Some(*value),
            _ => None,
        }
    }
}

impl<'a> FromPropertyValue<'a> for &'a [i32] {
    fn from_property_value(value: &'a PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::VecI32(value) => Some(value),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for Vec<i32> {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::VecI32(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl<'a> FromPropertyValue<'a> for &'a [f32] {
    fn from_property_value(value: &'a PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::VecF32(value) => Some(value),
            _ => None,
        }
    }
}

impl FromPropertyValue<'_> for Vec<f32> {
    fn from_property_value(value: &PropertyValue) -> Option<Self> {
        match value {
            PropertyValue::VecF32(value) => Some(value.clone()),
            _ => None,
        }
    }
}

use std::io::{Read, Seek, Write};

use binrw::{BinRead, BinWrite, binrw};
use bitflags::bitflags;
use jc2_hashing::HashString;

use crate::{common::NullString, math::Vec3};

#[binrw]
#[derive(Clone, Debug)]
pub struct ModelCollection {
    pub endian: ModelCollectionEndian,
    #[brw(is_little(endian.is_little()))]
    #[bw(args(models))]
    pub info: ModelCollectionInfo,
    #[brw(is_little(endian.is_little()), args(info.instance_count, info.version))]
    pub instances: ModelCollectionInstances,
    #[brw(is_little(endian.is_little()), args(info.grid_instance_count, info.grid_cell_count))]
    pub grid: ModelCollectionGrid,
    #[br(count(info.model_count))]
    #[bw(assert(models.len() as u32 == info.model_count))]
    pub models: Vec<NullString>,
    #[br(count(info.vegetation_instance_count))]
    #[bw(assert(vegetation_instances.len() as u32 == info.vegetation_instance_count))]
    #[brw(is_little(endian.is_little()))]
    pub vegetation_instances: Vec<VegetationInstance>,
}

impl ModelCollection {
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

    pub fn instances<'a>(&'a self) -> impl Iterator<Item = ModelInstance<'a>> {
        self.instances
            .transforms
            .iter()
            .zip(self.instances.models.iter())
            .enumerate()
            .map(|(index, (transform, model_index))| ModelInstance {
                transform: transform,
                model_index: *model_index,
                lod: self.instances.lods.get(index),
                flags: self.instances.flags.get(index).map(|&flags| {
                    if self.info.version == ModelCollectionVersion::V5 {
                        flags | ModelInstanceFlags::CLIP
                    } else {
                        flags
                    }
                }),
                bounds: self.instances.bounds.get(index),
            })
    }
}

#[binrw]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelCollectionEndian {
    #[brw(magic = b"\x14\x03\x03\x83")]
    Little,
    #[brw(magic = b"\x83\x00\x00\x14")]
    Big,
}

impl ModelCollectionEndian {
    fn is_little(&self) -> bool {
        matches!(self, ModelCollectionEndian::Little)
    }
}

#[binrw]
#[bw(import(models: &Vec<NullString>))]
#[derive(Clone, Debug)]
pub struct ModelCollectionInfo {
    pub version: ModelCollectionVersion,
    pub instance_count: u32,
    pub model_count: u32,
    #[bw(write_with = Self::write_models_size, args(models))]
    pub models_size: u32,
    #[brw(if(version.greater(ModelCollectionVersion::V1)))]
    pub density: u32,
    pub grid_instance_count: u32,
    pub grid_cell_count: u32,
    #[brw(if(version.greater(ModelCollectionVersion::V3)))]
    pub vegetation_instance_count: u32,
    pub min: Vec3<f32>,
    #[brw(if(version.less(ModelCollectionVersion::V5)))]
    pub unknown0: u32,
    pub max: Vec3<f32>,
    #[brw(if(version.less(ModelCollectionVersion::V5)))]
    pub unknown1: [u32; 2],
}

impl ModelCollectionInfo {
    #[binrw::writer(writer, endian)]
    fn write_models_size(_: &u32, models: &Vec<NullString>) -> binrw::BinResult<()> {
        let value = models
            .iter()
            .fold(0u32, |size, name| size + name.size() as u32);
        value.write_options(writer, endian, ())?;
        Ok(())
    }
}

#[binrw]
#[brw(repr = u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub enum ModelCollectionVersion {
    V1 = 1,
    V2,
    V3,
    V4,
    V5,
    V6,
    #[default]
    V7,
}

impl ModelCollectionVersion {
    fn greater(&self, version: Self) -> bool {
        *self as u32 > version as u32
    }

    fn less(&self, version: Self) -> bool {
        version as u32 > *self as u32
    }
}

#[binrw]
#[brw(import(count: u32, version: ModelCollectionVersion))]
#[derive(Clone, Debug)]
pub struct ModelCollectionInstances {
    #[br(count(count))]
    #[bw(assert(transforms.len() as u32 == count))]
    pub transforms: Vec<[f32; 4 * 4]>,
    #[br(count(count))]
    #[bw(assert(models.len() as u32 == count))]
    pub models: Vec<u16>,
    #[br(count(count))]
    #[bw(assert(lods.len() as u32 == count))]
    #[brw(if(version.greater(ModelCollectionVersion::V2)))]
    pub lods: Vec<(Vec3<f32>, f32)>,
    #[br(count(count))]
    #[bw(assert(flags.len() as u32 == count))]
    #[brw(if(version.greater(ModelCollectionVersion::V4)))]
    pub flags: Vec<ModelInstanceFlags>,
    #[br(count(count))]
    #[bw(assert(bounds.len() as u32 == count))]
    #[brw(if(version.greater(ModelCollectionVersion::V6)))]
    pub bounds: Vec<(Vec3<f32>, Vec3<f32>)>,
}

bitflags! {
    #[binrw]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
    pub struct ModelInstanceFlags: u16 {
        const SHADOW = 1 << 0;
        const CLIP = 1 << 1;
    }
}

#[binrw]
#[brw(import(instance_count: u32, cell_count: u32))]
#[derive(Clone, Debug)]
pub struct ModelCollectionGrid {
    #[br(count(instance_count))]
    #[bw(assert(instances.len() as u32 == instance_count))]
    pub instances: Vec<u16>,
    #[br(count(cell_count))]
    #[bw(assert(cells.len() as u32 == cell_count))]
    pub cells: Vec<ModelCollectionGridCell>,
}

#[binrw]
#[brw(import(instance_count: u32, cell_count: u32))]
#[derive(Clone, Copy, Debug)]
pub struct ModelCollectionGridCell {
    pub offset: u16,
    pub counts: [u16; 3],
}

#[binrw]
#[derive(Clone, Debug)]
pub struct VegetationInstance {
    pub position: Vec3<f32>,
    pub model_hash: HashString,
    #[brw(pad_after = 3)]
    pub yaw: u8,
}

#[derive(Clone, Debug)]
pub struct ModelInstance<'a> {
    pub transform: &'a [f32; 4 * 4],
    pub model_index: u16,
    pub lod: Option<&'a (Vec3<f32>, f32)>,
    pub flags: Option<ModelInstanceFlags>,
    pub bounds: Option<&'a (Vec3<f32>, Vec3<f32>)>,
}

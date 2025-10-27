use binrw::binrw;

use crate::common::{LengthBitVec, LengthVec};

#[binrw]
#[brw(magic = 12u32)]
#[derive(Clone, Debug)]
pub struct TerrainFile {
    #[brw(magic = 123u32)]
    pub height_map: [u16; 132 * 132],
    pub material_map: [u8; 132 * 132],
    #[brw(magic = 12u32)]
    pub textures: TerrainTextureData,
    #[brw(magic = 12u32)]
    pub lods: TerrainChunkData,
    #[brw(magic = 12u32)]
    pub zone_map: [u8; 64 * 64],
    #[brw(magic = 12u32)]
    pub magic: (),
}

impl Default for TerrainFile {
    fn default() -> Self {
        Self {
            height_map: [Default::default(); 132 * 132],
            material_map: [Default::default(); 132 * 132],
            textures: Default::default(),
            lods: Default::default(),
            zone_map: [Default::default(); 64 * 64],
            magic: Default::default(),
        }
    }
}

#[binrw]
#[derive(Clone, Default, Debug)]
pub struct TerrainTextureData {
    pub normal_map: LengthVec<u8, u32>,
    pub material_map: LengthVec<u8, u32>,
    pub weight_map: LengthVec<u8, u32>,
    #[brw(magic = 12u32)]
    pub map_tile: LengthVec<u8, u32>,
}

#[binrw]
#[derive(Clone, Default, Debug)]
pub struct TerrainChunkData {
    pub high: [TerrainChunk; 1],
    pub medium: [TerrainChunk; 4],
    pub low: [TerrainChunk; 16],
}

#[binrw]
#[derive(Clone, Default, Debug)]
pub struct TerrainChunk {
    pub triangles: LengthBitVec<u32, u32>,
    pub index_counts: [u16; 4],
}

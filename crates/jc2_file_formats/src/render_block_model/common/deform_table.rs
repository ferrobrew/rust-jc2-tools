use binrw::binrw;

#[binrw]
#[derive(Clone, Debug)]
pub struct DeformTable {
    pub data: [u32; DeformTable::MAX_TABLE_SIZE],
}

impl DeformTable {
    pub const MAX_TABLE_SIZE: usize = 256;
}

impl Default for DeformTable {
    fn default() -> Self {
        Self {
            data: [0u32; DeformTable::MAX_TABLE_SIZE],
        }
    }
}

use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct DeformTable {
    pub data: [u16; DeformTable::MAX_TABLE_SIZE],
}

impl DeformTable {
    pub const MAX_TABLE_SIZE: usize = 256;
}

impl Default for DeformTable {
    fn default() -> Self {
        Self {
            data: [0u16; DeformTable::MAX_TABLE_SIZE],
        }
    }
}

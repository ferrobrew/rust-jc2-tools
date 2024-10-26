use std::path::Path;

use binrw::binrw;

use super::hash_little32;

#[binrw]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashString(u32);

impl HashString {
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn default() -> Self {
        Self(hash_little32(b""))
    }

    #[inline]
    pub const fn from_bytes(bytes: &[u8]) -> Self {
        Self(hash_little32(bytes))
    }

    #[inline]
    pub const fn from_str(str: &str) -> Self {
        Self(hash_little32(str.as_bytes()))
    }

    #[inline]
    pub fn from_path<P: AsRef<Path>>(path: &P) -> Option<Self> {
        let path = path.as_ref();
        path.file_name().filter(|a| a.is_ascii()).map(|a| {
            Self(hash_little32(
                a.to_ascii_lowercase().to_string_lossy().as_bytes(),
            ))
        })
    }

    #[inline]
    pub const fn hash(&self) -> u32 {
        self.0
    }

    #[inline]
    pub fn hash_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

impl Default for HashString {
    #[inline]
    fn default() -> Self {
        Self::default()
    }
}

impl From<&str> for HashString {
    #[inline]
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<String> for HashString {
    #[inline]
    fn from(value: String) -> Self {
        Self::from_bytes(value.as_bytes())
    }
}

impl From<u32> for HashString {
    #[inline]
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<HashString> for u32 {
    #[inline]
    fn from(value: HashString) -> Self {
        value.0
    }
}

const_assert_eq!(HashString::from_bytes(b"rico").hash(), 0x6041E481);
const_assert_eq!(HashString::from_str("jc2").hash(), 0xCDF21378);

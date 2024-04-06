use std::path::Path;

use super::hash_little32;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashString(u32);

impl HashString {
    #[inline(always)]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn default() -> Self {
        Self(hash_little32(b""))
    }

    #[inline(always)]
    pub const fn from_bytes(bytes: &[u8]) -> Self {
        Self(hash_little32(bytes))
    }

    #[inline(always)]
    pub const fn from_str(str: &str) -> Self {
        Self(hash_little32(str.as_bytes()))
    }

    #[inline(always)]
    pub fn from_path(path: &Path) -> Option<Self> {
        path.file_name().filter(|a| a.is_ascii()).map(|a| {
            Self(hash_little32(
                a.to_ascii_lowercase().to_string_lossy().as_bytes(),
            ))
        })
    }

    #[inline(always)]
    pub const fn hash(&self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub fn hash_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

impl Default for HashString {
    #[inline(always)]
    fn default() -> Self {
        Self::default()
    }
}

impl From<String> for HashString {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::from_bytes(value.as_bytes())
    }
}

impl From<u32> for HashString {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<HashString> for u32 {
    #[inline(always)]
    fn from(value: HashString) -> Self {
        value.0
    }
}

const_assert_eq!(HashString::from_bytes(b"rico").hash(), 0x6041E481);
const_assert_eq!(HashString::from_str("jc2").hash(), 0xCDF21378);

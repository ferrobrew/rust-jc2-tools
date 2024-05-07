use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::HashString;

type Iter<'a> = std::collections::hash_map::Iter<'a, HashString, HashEntry>;
type IterMut<'a> = std::collections::hash_map::IterMut<'a, HashString, HashEntry>;
type Keys<'a> = std::collections::hash_map::Keys<'a, HashString, HashEntry>;
type Values<'a> = std::collections::hash_map::Values<'a, HashString, HashEntry>;

#[derive(Debug, Clone)]
pub enum HashEntry {
    String(String),
    Path(PathBuf),
}

impl HashEntry {
    #[inline]
    fn as_string(&self) -> Option<&String> {
        match self {
            HashEntry::String(str) => Some(str),
            HashEntry::Path(_) => None,
        }
    }

    #[inline]
    fn as_path(&self) -> Option<&Path> {
        match self {
            HashEntry::Path(path) => Some(path),
            HashEntry::String(_) => None,
        }
    }
}

impl From<String> for HashEntry {
    #[inline]
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<PathBuf> for HashEntry {
    #[inline]
    fn from(value: PathBuf) -> Self {
        Self::Path(value)
    }
}

#[derive(Default, Debug, Clone)]
pub struct HashList(HashMap<HashString, HashEntry>);

impl HashList {
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    #[inline]
    pub fn extend<T: IntoIterator<Item = (HashString, HashEntry)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }

    #[inline]
    pub fn keys(&self) -> Keys {
        self.0.keys()
    }

    #[inline]
    pub fn values(&self) -> Values {
        self.0.values()
    }

    #[inline]
    pub fn iter(&self) -> Iter {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut {
        self.0.iter_mut()
    }

    #[inline]
    pub fn insert(&mut self, entry: HashEntry) {
        match entry {
            HashEntry::Path(path) => self.insert_path(path),
            HashEntry::String(string) => self.insert_string(string),
        }
    }

    #[inline]
    pub fn insert_string(&mut self, string: impl Into<String>) {
        let string: String = string.into();
        self.0.insert(HashString::from_str(&string), string.into());
    }

    #[inline]
    pub fn insert_path(&mut self, path: impl Into<PathBuf>) {
        let path: PathBuf = path.into();
        if let Some(hash) = HashString::from_path(&path) {
            self.0.insert(hash, path.into());
        }
    }

    #[inline]
    pub fn contains(&self, hash: HashString) -> bool {
        self.0.contains_key(&hash)
    }

    #[inline]
    pub fn find(&self, hash: HashString) -> Option<&HashEntry> {
        self.0.get(&hash)
    }

    #[inline]
    pub fn find_string(&self, hash: HashString) -> Option<&String> {
        self.0.get(&hash).and_then(|v| v.as_string())
    }

    #[inline]
    pub fn find_path(&self, hash: HashString) -> Option<&Path> {
        self.0.get(&hash).and_then(|v| v.as_path())
    }

    #[inline]
    pub fn remove(&mut self, hash: HashString) {
        self.0.remove(&hash);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<'a> IntoIterator for &'a HashList {
    type IntoIter = Iter<'a>;
    type Item = (&'a HashString, &'a HashEntry);

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut HashList {
    type IntoIter = IterMut<'a>;
    type Item = (&'a HashString, &'a mut HashEntry);

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl FromIterator<(HashString, HashEntry)> for HashList {
    fn from_iter<T: IntoIterator<Item = (HashString, HashEntry)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl<const N: usize> From<[(HashString, HashEntry); N]> for HashList {
    fn from(arr: [(HashString, HashEntry); N]) -> Self {
        Self::from_iter(arr)
    }
}

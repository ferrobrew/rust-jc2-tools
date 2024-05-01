use std::{collections::HashMap, path::PathBuf};

use crate::HashString;

pub enum HashEntry {
    String(String),
    Path(PathBuf),
}

impl HashEntry {
    #[inline]
    fn as_string(&self) -> Option<&String> {
        match self {
            HashEntry::String(str) => Some(str),
            _ => None,
        }
    }

    #[inline]
    fn as_path(&self) -> Option<&PathBuf> {
        match self {
            HashEntry::Path(path) => Some(path),
            _ => None,
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

pub struct HashList(HashMap<HashString, HashEntry>);

impl HashList {
    #[inline]
    pub fn insert(&mut self, entry: HashEntry) {
        match entry {
            HashEntry::Path(path) => self.insert_path(path),
            HashEntry::String(string) => self.insert_string(string),
        }
    }

    #[inline]
    pub fn insert_string<S: Into<String>>(&mut self, string: S) {
        let string: String = string.into();
        self.0.insert(HashString::from_str(&string), string.into());
    }

    #[inline]
    pub fn insert_path<P: Into<PathBuf>>(&mut self, path: P) {
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
    pub fn find_path(&self, hash: HashString) -> Option<&PathBuf> {
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

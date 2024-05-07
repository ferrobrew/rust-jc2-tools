use std::path::{Path, PathBuf};

#[derive(Clone, Default, Debug)]
pub struct FileSystemTree(Vec<FileSystemTreeNode>);

impl FileSystemTree {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    #[inline]
    pub fn iter(&self) -> FileSystemTreeIter {
        FileSystemTreeIter {
            path: PathBuf::default(),
            tree: self,
            index: 0,
        }
    }

    #[inline]
    pub fn sort(&mut self) {
        for node in &mut self.0 {
            node.tree.sort();
        }
        self.0.sort_by(node_sort);
    }

    #[inline]
    pub fn insert(&mut self, path: &Path) {
        let Some(node) = FileSystemTreeNode::from_path(path) else {
            return;
        };
        self.insert_node(node);
    }

    #[inline]
    fn insert_node(&mut self, target: FileSystemTreeNode) {
        match self.0.iter_mut().find(|node| node.name == target.name) {
            Some(node) => {
                for target_node in target.tree.0 {
                    node.tree.insert_node(target_node);
                }
            }
            None => self.0.push(target),
        }
    }

    #[inline]
    pub fn remove(&mut self, path: &Path) {
        let Some(target) = FileSystemTreeNode::from_path(path) else {
            return;
        };
        self.remove_node(&target);
    }

    #[inline]
    fn remove_node(&mut self, target: &FileSystemTreeNode) {
        self.0.retain_mut(|node| {
            if node.name == target.name {
                for target_node in &target.tree.0 {
                    node.tree.remove_node(target_node);
                }
                !target.tree.0.is_empty()
            } else {
                true
            }
        });
    }
}

#[inline(always)]
fn node_sort(a: &FileSystemTreeNode, b: &FileSystemTreeNode) -> std::cmp::Ordering {
    lexical_sort::natural_lexical_cmp(&a.name, &b.name)
}

impl<'a> IntoIterator for &'a FileSystemTree {
    type IntoIter = FileSystemTreeIter<'a>;
    type Item = FileSystemTreeIterValue<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Debug)]
pub struct FileSystemTreeNode {
    name: String,
    tree: FileSystemTree,
}

impl FileSystemTreeNode {
    #[inline]
    fn from_path(path: &Path) -> Option<Self> {
        path.components().rev().fold(None, |previous, component| {
            let name = component.as_os_str().to_string_lossy().into();
            Some(Self {
                name,
                tree: FileSystemTree(previous.into_iter().collect()),
            })
        })
    }
}

pub struct FileSystemTreeIter<'a> {
    path: PathBuf,
    tree: &'a FileSystemTree,
    index: usize,
}

impl<'a> Iterator for FileSystemTreeIter<'a> {
    type Item = FileSystemTreeIterValue<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.tree.0.get(self.index).map(|node| {
            self.index += 1;
            FileSystemTreeIterValue {
                path: self.path.join(&node.name),
                node,
            }
        })
    }
}

pub struct FileSystemTreeIterValue<'a> {
    path: PathBuf,
    node: &'a FileSystemTreeNode,
}

impl<'a> FileSystemTreeIterValue<'a> {
    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.node.name
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.node.tree.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> FileSystemTreeIter<'a> {
        FileSystemTreeIter {
            path: self.path.clone(),
            tree: &self.node.tree,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &FileSystemTreeIterValue<'a> {
    type IntoIter = FileSystemTreeIter<'a>;
    type Item = FileSystemTreeIterValue<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

use std::path::{Path, PathBuf};

#[derive(Clone, Default, Debug)]
pub struct FileSystemTree(Vec<FileSystemTreeNode>);

impl FileSystemTree {
    pub fn iter(&self) -> FileSystemTreeIter {
        FileSystemTreeIter {
            path: PathBuf::default(),
            tree: self,
            index: 0,
        }
    }

    pub fn insert(&mut self, path: &Path) {
        let Some(node) = FileSystemTreeNode::from_path(path) else {
            return;
        };
        self.insert_node(node);
    }

    fn insert_node(&mut self, target: FileSystemTreeNode) {
        match self.0.iter_mut().find(|node| node.name == target.name) {
            Some(node) => {
                for target_node in target.tree.0 {
                    node.tree.insert_node(target_node);
                }
                node.tree.0.sort_by(node_sort);
            }
            None => self.0.push(target),
        }
    }

    pub fn remove(&mut self, path: &Path) {
        let Some(target) = FileSystemTreeNode::from_path(path) else {
            return;
        };
        self.remove_node(&target);
    }

    fn remove_node(&mut self, target: &FileSystemTreeNode) {
        self.0.retain_mut(|node| {
            if node.name == target.name {
                for target_node in &target.tree.0 {
                    node.tree.remove_node(target_node);
                }
                node.tree.0.sort_by(node_sort);
                !target.tree.0.is_empty()
            } else {
                true
            }
        });
    }
}

fn node_sort(a: &FileSystemTreeNode, b: &FileSystemTreeNode) -> std::cmp::Ordering {
    lexical_sort::natural_lexical_cmp(&a.name, &b.name)
}

impl<'a> IntoIterator for &'a FileSystemTree {
    type IntoIter = FileSystemTreeIter<'a>;
    type Item = FileSystemTreeIterValue<'a>;

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
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.node.name
    }

    pub fn is_empty(&self) -> bool {
        self.node.tree.0.is_empty()
    }

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

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

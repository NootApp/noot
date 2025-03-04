use serde_derive::{Serialize, Deserialize};

use std::path::PathBuf;

#[derive(Serialize,Deserialize, Debug, PartialEq)]
pub enum FileEntry {
    Folder(FileTree),
    File(String),
    Symlink(String),
}

#[derive(Serialize,Deserialize, Debug, PartialEq)]
pub struct FileTree {
    pub node_count: u32,
    pub parent: String,
    pub name: String,
    pub children: Vec<FileEntry>
}



impl FileTree {
    pub fn from_path(path: &PathBuf) -> Result<FileTree, std::io::Error> {
        let mut tree = FileTree {
            node_count: 0,
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            parent: path.parent().unwrap().to_str().unwrap().to_string(),
            children: vec!()
        };

        let mut entries = std::fs::read_dir(path)?;

        while let Some(Ok(entry)) = entries.next() {
            let metadata = entry.file_type()?;

            if metadata.is_dir() {
                let subtree = FileTree::from_path(&entry.path())?;

                tree.node_count = tree.node_count + subtree.node_count;
                tree.children.push(FileEntry::Folder(subtree));
            } else if metadata.is_file() {
                tree.node_count = tree.node_count + 1;
                tree.children.push(FileEntry::File(entry.file_name().to_str().unwrap().to_string()))
            } else if metadata.is_symlink() {
                tree.node_count = tree.node_count + 1;
                tree.children.push(FileEntry::Symlink(entry.file_name().to_str().unwrap().to_string()))
            }
        }

        Ok(tree)
    }

    pub fn from_path_str<A: Into<String>>(path: A) -> Result<FileTree, std::io::Error> {
        let pathbuf = PathBuf::from(path.into());
        FileTree::from_path(&pathbuf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_tree_recursion() {
        let tree = FileTree::from_path(&PathBuf::from("./static")).unwrap();
        dbg!(&tree);
        assert_eq!(tree.node_count, 6);
    }
}
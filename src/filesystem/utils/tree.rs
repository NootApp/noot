use serde_derive::{Deserialize, Serialize};

use std::path::PathBuf;
use iced::{color, Element, Padding};
use iced::widget::{button, column, scrollable, text};
use crate::app::GlobalEvent;
use crate::windows::editor_window::EditorEvent;
use ignore::{types, Walk};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FileEntry {
    Folder(FileTree),
    File(String, String),
    Symlink(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileTree {
    pub node_count: u32,
    pub breadcrumb: String,
    pub parent: String,
    pub name: String,
    pub children: Vec<FileEntry>,
}

impl FileTree {
    pub fn from_path(path: &PathBuf, use_ignore: bool) -> Result<FileTree, std::io::Error> {
        let mut tree = FileTree {
            node_count: 0,
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            breadcrumb: path.to_str().unwrap().to_string(),
            parent: path.parent().unwrap().to_str().unwrap().to_string(),
            children: vec![],
        };

        if use_ignore {
            // for result in Walk::new(path) {
            //     match result {
            //         Ok(entry) => {
            //
            //         }
            //     }
            // }
        } else {
            let mut entries = std::fs::read_dir(path)?;

            while let Some(Ok(entry)) = entries.next() {
                let metadata = entry.file_type()?;

                if metadata.is_dir() {
                    let subtree = FileTree::from_path(&entry.path(), false)?;

                    tree.node_count = tree.node_count + subtree.node_count;
                    tree.children.push(FileEntry::Folder(subtree));
                } else if metadata.is_file() {
                    tree.node_count = tree.node_count + 1;
                    tree.children.push(FileEntry::File(
                        entry.file_name().to_str().unwrap().to_string(), entry.path().to_str().unwrap().to_string(),
                    ))
                } else if metadata.is_symlink() {
                    tree.node_count = tree.node_count + 1;
                    tree.children.push(FileEntry::Symlink(
                        entry.file_name().to_str().unwrap().to_string(),
                    ))
                }
            }
        }

        Ok(tree)
    }

    pub fn from_path_str<A: Into<String>>(
        path: A,
    ) -> Result<FileTree, std::io::Error> {
        let pathbuf = PathBuf::from(path.into());
        FileTree::from_path(&pathbuf, false)
    }

    pub fn view(&self, recursive: bool) -> Element<GlobalEvent> {
        if !recursive {
            scrollable(
                column(self.children.iter().map(|e| e.view(true))).padding(Padding {
                    top: 5.,
                    bottom: 0.,
                    left: 20.,
                    right: 0.,
                })
            ).into()
        } else {
            column(self.children.iter().map(|e| e.view(true))).padding(Padding {
                top: 5.,
                bottom: 0.,
                left: 20.,
                right: 0.,
            }).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_tree_recursion() {
        let tree = FileTree::from_path(&PathBuf::from("./static"), false).unwrap();
        dbg!(&tree);
        assert_eq!(tree.node_count, 6);
    }
}


impl FileEntry {
    pub fn view(&self, recursive: bool) -> Element<GlobalEvent> {
        match self {
            FileEntry::Folder(subtree) => {
                column!(
                    text(subtree.name.clone()).color(color!(0x0000FF)),
                    subtree.view(true)
                ).into()
            }
            FileEntry::File(name, path) => {
                button(text(name).color(color!(0xFF0000))).padding(0).on_press_with(|| {
                    GlobalEvent::EditorBeam(EditorEvent::OpenFile(PathBuf::from(path.clone())))
                }).into()
            }
            FileEntry::Symlink(name) => {
                text(name).color(color!(0x00FF00)).into()
            }
        }
    }
}
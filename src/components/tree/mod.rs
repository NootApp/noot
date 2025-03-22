use std::path::PathBuf;
use iced::Element;
use iced::widget::{column, span, text};
use crate::app::GlobalEvent;
use crate::filesystem::utils::tree::{FileEntry, FileTree};

#[derive(Debug, Clone)]
pub struct TreeWidget {
    pub cwd: PathBuf,
    pub tree: FileTree
}

impl TreeWidget {
    pub fn new<P: Into<PathBuf>>(wd: P) -> Self {
        let cwd = wd.into();
        Self {
            cwd: cwd.clone(),
            tree: FileTree::from_path(&cwd).unwrap()
        }
    }

    pub fn cd<P: Into<PathBuf>>(&mut self, path: P) {
        self.cwd = path.into();
    }

    pub fn build_tree(&mut self) {
        self.tree = FileTree::from_path(&self.cwd).unwrap()
    }

    pub fn view(&self) -> Element<GlobalEvent> {
        column([
            text("Workspace Files").into(),
            self.tree.view(false).into(),
        ]).into()
    }

    pub fn has_readme(&self) -> Option<PathBuf> {
        let mut readme = None;

        for entry in &self.tree.children {
            match entry {
                FileEntry::Folder(_) => continue,
                FileEntry::File(content) => {
                    if content.to_lowercase().contains("readme.md") {
                        readme = Some(PathBuf::from(content));
                        break;
                    }
                    continue;
                }
                FileEntry::Symlink(_) => continue,
            }
        }


        readme
    }
}
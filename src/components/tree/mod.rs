use std::path::PathBuf;
use iced::Element;
use iced::widget::{column, span, text};
use crate::app::GlobalEvent;
use crate::filesystem::utils::tree::FileTree;

#[derive(Debug, Clone)]
pub struct TreeWidget {
    cwd: PathBuf,
    tree: FileTree
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
}
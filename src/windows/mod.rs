use crate::windows::build_info_window::BuildInfoWindow;
use crate::windows::editor_window::EditorWindow;

pub mod editor_window;
pub mod build_info_window;

#[derive(Debug)]
pub enum AppWindow {
    Editor(Box<EditorWindow>),
    BuildInfo(Box<BuildInfoWindow>)
}
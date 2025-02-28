use iced::widget::{
    self, button, center, 
    column, container, horizontal_space, 
    pick_list, row, text, text_editor, 
    toggler, tooltip
};

use iced::keyboard;
use iced::highlighter;
use iced::{Center, Element, Fill, Font, Task, Theme};

use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;


#[derive(Debug, Clone)]
pub enum Message {
    CreateNewWorkspace,

    /// Emitted when the content of a form element changes.
    /// Contains the ID of the form field which was changed, as well as the new content
    FormContentChanged(String, String),



    ActionPerformed(text_editor::Action),
    ThemeSelected(highlighter::Theme),
    WordWrapToggled(bool),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
}


#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}


use crate::filesystem::config::Config;
use crate::filesystem::workspace::state::WorkspaceState;
use std::io;

#[derive(Debug, Clone)]
pub enum Message {
    ///
    ConfigLoaded(Config),
    // CreateNewWorkspace,
    /// Emitted when the content of a form element changes.
    /// Contains the ID of the form field which was changed, as well as the new content
    FormContentChanged(String, String),
    
    WorkspaceLoaded(WorkspaceState)
    
    // ActionPerformed(text_editor::Action),
    // ThemeSelected(highlighter::Theme),
    // WordWrapToggled(bool),
    // NewFile,
    // OpenFile,
    // FileOpened(Result<(PathBuf, Arc<String>), Error>),
    // SaveFile,
    // FileSaved(Result<PathBuf, Error>),
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

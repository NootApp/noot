use std::path::PathBuf;
use iced::window::Id;
use crate::runtime::messaging::{Message, WindowMessage, WindowMessageKind};
use crate::storage::workspace::buffer::Buffer;

#[derive(Clone, Debug)]
pub enum EditorMessageKind {
    Tick,
    FileChanged(PathBuf),
    BufferRendered(Buffer),
    LoadAsset,
}

#[derive(Clone, Debug)]
pub struct EditorMessage {
    pub kind: EditorMessageKind,
    pub source_id: Option<Id>
}

impl EditorMessage {
    pub fn new(kind: EditorMessageKind, source_id: Id) -> EditorMessage {
        Self {
            kind, source_id: Some(source_id)
        }
    }
}

impl Into<Message> for EditorMessage {
    fn into(self) -> Message {
        WindowMessage {
            kind: WindowMessageKind::Editor(self.clone()),
            source_id: self.source_id,
        }.into()
    }
}

use iced::system;
use iced::window::Id;
use crate::hotkey::Keybind;
use crate::runtime::Task;
use crate::runtime::windows::editor::messaging::EditorMessage;
use crate::runtime::windows::workspace::WorkspaceWindowMessage;
use crate::runtime::windows::splash::SplashWindowMessage;
use crate::runtime::workers::Job;
use crate::utils::components::ComponentMessage;

#[derive(Debug, Clone)]
pub enum MessageKind {
    Tick,
    WindowOpen(String),
    WindowClose(Id),
    WindowMessage(WindowMessage),
    Component(ComponentMessage),
    Keybind(Keybind),
    OpenWorkspace(String),
    LinkOpened(Option<String>),
    Queue(Vec<Job>),
    SysInfo(system::Information),
}

#[derive(Debug, Clone)]
pub struct Message {
    pub kind: MessageKind,
    pub source_id: Option<Id>,
}

impl Message {
    pub fn new(kind: MessageKind, source_id: Option<Id>) -> Self {
        Self {
            kind,
            source_id,
        }
    }

    pub fn tick() -> Self {
        Self::new(MessageKind::Tick, None)
    }

    pub fn window_open(title: impl Into<String>) -> Self {
        Self::new(MessageKind::WindowOpen(title.into()), None)
    }

    pub fn window_close(id: Id) -> Self {
        Self::new(MessageKind::WindowClose(id), Some(id))
    }

    pub fn component(msg: ComponentMessage) -> Message {
        Self::new(MessageKind::Component(msg), None)
    }

    pub fn hotkey(event: Keybind) -> Message {
        Self::new(MessageKind::Keybind(event), None)
    }

    pub fn open_workspace(title: impl Into<String>) -> Self {
        Self::new(MessageKind::OpenWorkspace(title.into()), None)
    }
}

impl Into<Task> for Message {
    fn into(self) -> Task {
        Task::done(self)
    }
}

#[derive(Debug, Clone)]
pub enum WindowMessageKind {
    Splash(SplashWindowMessage),
    Debug(DebugMessage),
    Editor(EditorMessage),
    Component(ComponentMessage),
    Workspace(WorkspaceWindowMessage),
}

#[derive(Debug, Clone)]
pub struct WindowMessage {
    pub kind: WindowMessageKind,
    pub source_id: Option<Id>,
}

impl Into<Message> for WindowMessage {
    fn into(self) -> Message {
        Message {
            kind: MessageKind::WindowMessage(self.clone()),
            source_id: self.source_id,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DebugMessageKind {
    Log(String),
    Store(String, String)
}

#[derive(Clone, Debug)]
pub struct DebugMessage {
    pub kind: DebugMessageKind,
    pub source_id: Option<Id>,
}

impl Into<Message> for DebugMessage {
    fn into(self) -> Message {
        WindowMessage {
            kind: WindowMessageKind::Debug(self.clone()),
            source_id: self.source_id,
        }.into()
    }
}

use std::sync::{Arc, Mutex};
use iced::{window, Size, Subscription, Task as IcedTask, Theme};
use iced::widget::text;
use iced::window::{Id, Position, Settings};
use rust_i18n::t;
use crate::consts::{APP_ICON, APP_NAME};
use crate::runtime::{AppState, Element, Task, GLOBAL_STATE};
use crate::runtime::messaging::Message;
use crate::runtime::windows::DesktopWindow;
use crate::runtime::windows::editor::messaging::{EditorMessage, EditorMessageKind};
use crate::runtime::windows::editor::settings::EditorSettings;
use crate::storage::workspace::{WorkspaceManager};

pub mod settings;
pub mod messaging;

#[derive(Debug)]
pub struct EditorWindow {
    pub id: Id,
    state: Arc<Mutex<AppState>>,
    pub mgr: WorkspaceManager,
    pub settings: EditorSettings,
}


impl EditorWindow {
    pub fn new(mut mgr: WorkspaceManager) -> (Self, IcedTask<Id>) {
        let (id, task) = window::open(Self::settings());
        let window = Self {
            id,
            state: GLOBAL_STATE.clone(),
            mgr,
            settings: EditorSettings::new()
        };
        (
            window,
            task
        )
    }

    pub fn emit(&self, kind: EditorMessageKind) -> Message {
        EditorMessage::new(kind, self.id).into()
    }

    // pub fn subscription(&self) -> Subscription<Message> {
    //
    //     Subscription::run(watch_dir)
    // }
}

impl DesktopWindow<EditorWindow, EditorMessage, Message> for EditorWindow {
    fn settings() -> Settings {
        Settings {
            size: Size::new(1280., 720.),
            position: Position::Centered,
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: Some(window::icon::from_file_data(APP_ICON, None).unwrap()),
            platform_specific: Default::default(),
            exit_on_close_request: true,
        }
    }

    fn title(&self) -> String {
        t!("windows.editor.title", name=APP_NAME, workspace=self.mgr.source.name).into()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }

    fn update(&mut self, message: EditorMessage) -> Task {
        Task::none()
    }

    fn view(&self) -> Element {
        text(format!("{:?}", self.state)).into()
    }

    fn close(&mut self) -> Task {
        Task::none()
    }
}
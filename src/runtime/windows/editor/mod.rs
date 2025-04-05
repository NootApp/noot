use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use iced::{window, Size, Task as IcedTask, Theme};
use iced::widget::{text, container, row, column, scrollable};
use iced::window::{Id, Position, Settings};
use iced::{Length, color, Padding};
use iced::Subscription;
use rust_i18n::t;

use crate::consts::{APP_ICON, APP_NAME};
use crate::runtime::{AppState, Element, Task, GLOBAL_STATE};
use crate::runtime::messaging::Message;
use crate::runtime::windows::DesktopWindow;
use crate::storage::workspace::WorkspaceManager;
use crate::utils::components::widgets::status_bar::StatusBarWidget;

use self::buffer::Buffer;
use self::messaging::{EditorMessage, EditorMessageKind};
use self::settings::EditorSettings;

pub mod settings;
pub mod messaging;
pub mod buffer;
    

pub struct EditorWindow {
    pub id: Id,
    state: Arc<Mutex<AppState>>,
    pub mgr: WorkspaceManager,
    pub settings: EditorSettings,
    pub widgets: Vec<Box<dyn StatusBarWidget>>,
    pub buffers: Vec<Buffer>
}

impl Debug for EditorWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{\n\tid: {:?},\n\tstate: {:?},\n\tmgr: {:?},\n\tsettings: {:?},\n\twidgets: Vec<Length={}>\n}}", self.id, self.state, self.mgr, self.settings, self.widgets.len()))
    }
}


impl EditorWindow {
    pub fn new(mgr: WorkspaceManager) -> (Self, IcedTask<Id>) {
        let (id, task) = window::open(Self::settings());
        let window = Self {
            id,
            state: GLOBAL_STATE.clone(),
            mgr,
            settings: EditorSettings::new(),
            widgets: vec![],
            buffers: vec![
                Buffer::from_md("Test".to_string(), "noot://internal/test", include_str!("../../../../static/experiences/test.md").to_string())
            ],
        };

        (
            window,
            task
        )
    }

    pub fn register_status_widget<W: StatusBarWidget + 'static>(&mut self, widget: W) {
        self.widgets.push(Box::new(widget));
    }

    pub fn emit(&self, kind: EditorMessageKind) -> Message {
        EditorMessage::new(kind, self.id).into()
    }

    pub fn subscribe(&self) -> Subscription<Message> {
        let mut subscriptions = vec![];


        for widget in self.widgets.iter() {
            subscriptions.push(widget.subscribe());
        }


        Subscription::batch(subscriptions)
    }
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

    fn update(&mut self, _message: EditorMessage) -> Task {
        Task::none()
    }

    fn view(&self) -> Element {
        let mut status_bar_padding = Padding::new(5.);
        status_bar_padding.left = 10.;
        status_bar_padding.right = 10.;

        column!(
            container(
                text("Status Bar")
            ).width(Length::Fill).height(30).padding(status_bar_padding).style(|_| {
                    container::Style::default()
                        .background(color!(0xa30000))
                }),
            row!(
                container(
                    scrollable(
                        text("File List")
                    )
                ).width(250),
                container(
                    column!(
                        row!(
                            text("tab bar")
                        ),
                        scrollable(
                            self.buffers[0].view()
                        )
                    )
                )
            )
        ).into()
    }

    fn close(&mut self) -> Task {
        Task::none()
    }
}

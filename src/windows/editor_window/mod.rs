use std::collections::BTreeMap;
use std::fs::read;
use std::io::read_to_string;
use std::path::PathBuf;
use iced::{window, Element, Size, Task, Theme};
use iced::widget::{row, Container, text, column, markdown, Row};
use iced::widget::markdown::Url;
use iced::window::{icon, Id, Settings};
use crate::app::GlobalEvent;
use crate::components::tree::TreeWidget;
use crate::consts::APP_ICON;
use crate::filesystem::workspace::state::WorkspaceState;
use crate::ViewPort;


#[derive(Debug)]
pub struct EditorWindow {
    pub id: Id,
    pub workspace: WorkspaceState,
    pub file_list: TreeWidget,
    pub editor: Vec<markdown::Item>,
    pub theme: Theme,
    #[cfg(debug_assertions)]
    pub debug: BTreeMap<String, String>,
}


#[derive(Debug, Clone)]
pub enum EditorEvent {
    OpenFile(PathBuf),
    Ready,
    LinkClicked(Url),
    Debug(String, String),
}


impl EditorWindow {
    pub fn new(state: WorkspaceState, theme: Theme) -> (Self, Id, Task<Id>) {
        let (id, task) = window::open(Self::config());

        let window = Self {
            id,
            workspace: state.clone(),
            file_list: TreeWidget::new(state.manifest.parse_local_path().unwrap()),
            editor: vec![],
            theme,
            #[cfg(debug_assertions)]
            debug: BTreeMap::from([("Ready".to_string(), "False".to_string())]),
        };

        (window, id, task)
    }

    pub fn config() -> Settings {
        Settings {
            size: Size { width: 1920., height: 1000. },
            position: Default::default(),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: Some(icon::from_file_data(APP_ICON, None).unwrap()),
            platform_specific: Default::default(),
            exit_on_close_request: true,
        }
    }

    pub fn update(&mut self, event: EditorEvent) -> Task<GlobalEvent> {
        warn!("EditorEvent: {:?}", event);
        match event {
            EditorEvent::Ready => {
                let default_buffer = self.file_list.has_readme();

                if let Some(file) = default_buffer {
                    Task::done(GlobalEvent::Editor(self.id, EditorEvent::OpenFile(file))).chain(self.debug("Ready", "true"))
                } else {
                    self.debug("Ready", "true")
                }
            }
            EditorEvent::OpenFile(path) => {
                let buffer = String::from_utf8(read(path.clone()).unwrap()).unwrap();
                self.editor = markdown::parse(&buffer).collect();
                dbg!(&self);
                self.debug("current_file", path.to_str().unwrap())
            }
            EditorEvent::Debug(key, value) => {
                self.debug.insert(key, value);
                Task::none()
            }
            _ => {
                debug!("Unknown update event: {:?}", event);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<GlobalEvent> {
        Container::new(
            row!(
                self.file_list.view(),
                column!(
                    text("Editor Section"),
                    markdown::view(
                        &self.editor,
                        markdown::Settings::default(),
                        markdown::Style::from_palette(self.theme().palette())
                    )
                    .map(|e| GlobalEvent::Editor(self.id, EditorEvent::LinkClicked(e)))
                ),
                column!(
                    text("Right Utility Bar"),

                    #[cfg(debug_assertions)]
                    self.render_debug()
                )
            )
        ).into()
    }

    #[cfg(debug_assertions)]
    pub fn render_debug(&self) -> Element<GlobalEvent> {
        column(
            self.debug.iter().map(|(k,v)| {
                row!(text(k), text(v)).into()
            }),
        ).into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn debug<S: Into<String>>(&self, key: S, value: S) -> Task<GlobalEvent> {
        Task::done(
            GlobalEvent::Editor(
                self.id,
                EditorEvent::Debug(
                    key.into(),
                    value.into()
                )
            )
        )
    }
}

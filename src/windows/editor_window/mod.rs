use std::collections::BTreeMap;
use std::fs::read;
use std::path::PathBuf;
use iced::{application, window, Element, Length, Size, Task, Theme};
use iced::widget::{row, Container, text, column, markdown, Row, Checkbox, scrollable};
use iced::widget::markdown::Url;
use iced::window::{icon, Id, Settings};
use pulldown_cmark::{Event, TextMergeStream};
use crate::app::GlobalEvent;
use crate::components::md::{Kind, MarkdownToken};
use crate::components::tree::TreeWidget;
use crate::consts::{APP_ICON, APP_NAME, APP_VERSION};
use crate::filesystem::workspace::state::WorkspaceState;


#[derive(Debug)]
pub struct EditorWindow {
    pub id: Id,
    pub workspace: WorkspaceState,
    pub file_list: TreeWidget,
    pub editor: Vec<MarkdownToken>,
    pub theme: Theme,
    pub title: String,
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
            title: format!("{} - {}", APP_NAME, APP_VERSION),
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
                let buffer = read(path.clone()).unwrap();

                let text = String::from_utf8(buffer);

                if let Ok(text) = text {
                    self.editor = vec![];
                    let parser = TextMergeStream::new(pulldown_cmark::Parser::new(&text));
                    let mut current = MarkdownToken::new(Kind::SoftBreak);
                    let mut has_ended = true;
                    for event in parser {
                        match event {
                            Event::Start(tag) => {
                                if !has_ended {

                                }
                            }
                            Event::Text(content) => {
                                info!("{:?}", content);
                                let mut token = MarkdownToken::new(Kind::Text);
                                token.content = Some(content.to_string());
                                self.editor.push(token);
                            },
                            // Event::
                            _ => {
                                warn!("Unsupported token type: {:?}", event);
                            }
                        }
                    }

                } else {
                    warn!("Unsupported content type found in file (not text): {}", path.display());
                }


                info!("Rendered file with the following editor data");
                dbg!(&self.editor);

                self.title = format!("{} - {} - Editing {}", APP_NAME, APP_VERSION, path.file_name().unwrap().to_str().unwrap());

                Task::batch([
                    self.debug("workspace_name", &*self.workspace.manifest.name.clone().unwrap_or("Unknown".to_string())),
                    self.debug("workspace_path", self.workspace.manifest.parse_local_path().unwrap().to_str().unwrap()),
                    self.debug("current_file", path.to_str().unwrap())
                ])
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
                scrollable(
                    column(self.editor.iter().map(|token| token.view())).width(Length::Fill)
                ),
                #[cfg(debug_assertions)]
                column!(
                    text("Right Utility Bar"),
                    self.render_debug()
                ).width(600.0)
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

    pub fn title(&self) -> String {
        self.title.clone()
    }
}

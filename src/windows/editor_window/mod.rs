use std::collections::BTreeMap;
use std::fs::read;
use std::ops::Add;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use discord_rich_presence::activity::{Activity, ActivityType, Timestamps};
use iced::{window, Element, Length, Size, Task, Theme};
use iced::widget::{row, Container, text, column, scrollable, container, text_editor};
use iced::widget::markdown::Url;
use iced::widget::text_editor::Action;
use iced::window::{icon, Id, Settings};
use pulldown_cmark::{Event, Tag, TextMergeStream};
use crate::app::GlobalEvent;
use crate::components::md::{Kind, MarkdownToken};
use crate::components::table::row::TableRow;
use crate::components::table::Table;
use crate::components::tree::TreeWidget;
use crate::consts::{APP_ICON, APP_NAME, APP_VERSION};
use crate::filesystem::workspace::state::WorkspaceState;


#[derive(Debug)]
pub struct EditorWindow {
    pub id: Id,
    pub workspace: WorkspaceState,
    pub current_file: PathBuf,
    pub current_file_name: String,
    pub file_list: TreeWidget,
    pub preview: Vec<MarkdownToken>,
    pub editor: text_editor::Content,

    pub theme: Theme,
    pub title: String,
    pub last_file_change: DateTime<Local>,
    pub last_interaction: DateTime<Local>,

    #[cfg(debug_assertions)]
    pub debug: BTreeMap<String, String>,

    #[cfg(debug_assertions)]
    pub dbg_table: Table,
}


#[derive(Debug, Clone)]
pub enum EditorEvent {
    OpenFile(PathBuf),
    Ready,
    LinkClicked(Url),
    Debug(String, String),
    Edit(Action),
}


impl EditorWindow {
    pub fn new(state: WorkspaceState, theme: Theme) -> (Self, Id, Task<Id>) {
        let (id, task) = window::open(Self::config());

        let mut window = Self {
            id,
            workspace: state.clone(),
            current_file: Default::default(),
            current_file_name: "".to_string(),
            file_list: TreeWidget::new(state.manifest.parse_local_path().unwrap()),
            preview: vec![],
            editor: text_editor::Content::with_text("Nothing to see here"),
            theme,
            title: format!("{} - {}", APP_NAME, APP_VERSION),
            #[cfg(debug_assertions)]
            debug: BTreeMap::from([("Ready".to_string(), "False".to_string())]),
            #[cfg(debug_assertions)]
            dbg_table: Table::new(),
            last_file_change: Default::default(),
            last_interaction: Default::default(),
        };

        #[cfg(debug_assertions)]
        {
            window.dbg_table = window.dbg_table.headers(TableRow::new(vec!["Field", "Value"]));
        }



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
                    let mut nodes: Vec<MarkdownToken> = vec![];
                    let parser = pulldown_cmark::Parser::new(&text);
                    for event in parser {
                        match event {
                            Event::Start(tag) => {
                                match tag {
                                    Tag::Heading { level, id: _, classes: _, attrs: _ } => {
                                        nodes.push(MarkdownToken::new(Kind::Heading(level as usize)));
                                    },
                                    Tag::Paragraph => nodes.push(MarkdownToken::new(Kind::Paragraph)),
                                    _ => {
                                        warn!("Tag {:?} is unknown", tag);
                                    }
                                }
                            }
                            Event::Text(content) => {
                                let last = nodes.last_mut().unwrap();

                                match &mut last.content {
                                    Some( c) => last.content = Some(c.clone().add(content.to_string().as_str())),
                                    None => last.content = Some(content.to_string()),
                                }


                                // info!("{:?}", content);
                                // let mut token = MarkdownToken::new(Kind::Text);
                                // token.content = Some(content.to_string());
                                // self.editor.push(token);
                            },
                            // Event::
                            _ => {
                                warn!("Unsupported token type: {:?}", event);
                            }
                        }
                    }
                    self.editor = text_editor::Content::with_text(&text);
                    self.preview = nodes;
                    self.current_file = path.clone();
                    self.current_file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                    self.last_interaction = Local::now();
                    self.last_file_change = Local::now();
                } else {
                    warn!("Unsupported content type found in file (not text): {}", path.display());
                }


                info!("Rendered file with the following editor data");
                dbg!(&self.preview);

                self.title = format!("{} - {} - Editing {}", APP_NAME, APP_VERSION, path.file_name().unwrap().to_str().unwrap());

                Task::batch([
                    #[cfg(feature = "drpc")]
                    Task::done(GlobalEvent::UpdatePresence(self.id)),
                    self.debug("workspace_name", &*self.workspace.manifest.name.clone().unwrap_or("Unknown".to_string())),
                    self.debug("workspace_path", self.workspace.manifest.parse_local_path().unwrap().to_str().unwrap()),
                    self.debug("current_file", path.to_str().unwrap())
                ])
            }
            #[cfg(debug_assertions)]
            EditorEvent::Debug(key, value) => {
                self.debug.insert(key, value);
                self.redraw_debug();
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
                row!(
                    scrollable(
                        column!(text_editor(&self.editor).on_action(|e| {
                            GlobalEvent::Editor(self.id, EditorEvent::Edit(e))
                        })).width(Length::Fill)
                    ),
                    scrollable(
                        column(self.preview.iter().map(|token| token.view())).width(Length::Fill)
                    )
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
        scrollable(
            container(self.dbg_table.view())
                .padding(10.)
        ).into()
    }

    #[cfg(debug_assertions)]
    pub fn redraw_debug(&mut self) {
        self.dbg_table.rows.clear();

        for (key, val) in &self.debug {
            self.dbg_table.rows.push(
                TableRow::new(vec![key.clone(), val.clone()])
            )
        }
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

    #[cfg(feature = "drpc")]
    pub fn get_presence(&self) -> Activity {
        build_activity_from_state(&self)
    }
}


#[cfg(feature = "drpc")]
fn build_activity_from_state(w: &EditorWindow) -> Activity {
    Activity::new()
    .activity_type(ActivityType::Playing)
        .details(activity_details(w))
        .state(activity_state(w))
        .timestamps(activity_timestamps(w))
}

#[cfg(feature = "drpc")]
fn activity_details(w: &EditorWindow) -> &str {
    w.workspace.manifest.name.clone().unwrap_or("Unknown Workspace".to_string()).leak()
}

#[cfg(feature = "drpc")]
fn activity_state(w: &EditorWindow) -> &str {
    let now = Local::now();

    let (idle, _) = activity_is_idle(w);

    if idle {
        "idle"
    } else {
        format!("Working on '{}'", w.current_file_name).leak()
    }
}

#[cfg(feature = "drpc")]
fn activity_timestamps(w: &EditorWindow) -> Timestamps {
    let (idle, since) = activity_is_idle(w);

    if idle {
        Timestamps::new()
            .start(since)
    } else {
        Timestamps::new()
            .start(w.last_file_change.timestamp())
    }
}

#[cfg(feature = "drpc")]
fn activity_is_idle(w: &EditorWindow) -> (bool, i64) {
    let now = Local::now();
    let diff = now.timestamp() - w.last_interaction.timestamp();
    if diff > 30 {
        (true, w.last_interaction.timestamp())
    } else {
        (false, 0)
    }
}
use std::collections::BTreeMap;
use std::fs::read;
use std::ops::Add;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use discord_rich_presence::activity::{Activity, ActivityType, Timestamps};
use iced::{window, Color, Element, Length, Size, Task, Theme};
use iced::widget::{row, Container, text, column, scrollable, container, text_editor, image, horizontal_space, button};
use iced::widget::markdown::Url;
use iced::widget::shader::wgpu::core::global::Global;
use iced::widget::text_editor::Action;
use iced::window::{icon, Id, Settings};
use ignore::gitignore::Glob;
use pulldown_cmark::{Event, Tag, TagEnd, TextMergeStream};
use crate::app::GlobalEvent;
use crate::components::md::{Kind, MarkdownToken, TextToken};
use crate::components::table::row::TableRow;
use crate::components::table::Table;
use crate::components::tree::TreeWidget;
use crate::consts::{APP_ICON, APP_NAME, APP_VERSION};
use crate::filesystem::workspace::state::WorkspaceState;
use crate::markdown::TextModifier;

#[derive(Debug)]
pub struct EditorPanel {
    pub window_id: Id,
    pub preview: Vec<MarkdownToken>,
    pub editor: text_editor::Content,
    pub file: PathBuf,
    pub tab_title: String
}

impl EditorPanel {
    pub fn new(id: Id, path: PathBuf) -> Self {
        Self {
            window_id: id,
            preview: vec![],
            editor: text_editor::Content::new(),
            file: path.clone(),
            tab_title: path.file_name().unwrap().to_str().unwrap().to_string(),
        }
    }

    pub fn view(&self) -> Element<GlobalEvent> {
        row!(
            column!(text_editor(&self.editor).on_action(|e| {
                GlobalEvent::Editor(self.window_id, EditorEvent::Edit(self.file.clone(), e))
            }).height(Length::Fill)).width(Length::Fill).height(Length::Fill),
            scrollable(
                column(self.preview.iter().map(|token| token.view())).width(Length::Fill)
            ).height(Length::Fill),
        ).height(Length::Fill).into()
    }

    pub fn view_tab(&self) -> Element<GlobalEvent> {
        row!(
            button(
                row!(
                    horizontal_space(),
                    text(self.tab_title.as_str()),
                    horizontal_space(),
                )
            ).on_press_with(|| {
                GlobalEvent::Editor(self.window_id, EditorEvent::FocusFile(self.file.clone()))
            }),
            button(text("x").width(10.)).on_press_with(|| {
                GlobalEvent::Editor(self.window_id, EditorEvent::CloseFile(self.file.clone()))
            })
        ).width(150.).into()
    }

    pub fn close(&self) {} // currently does nothing
}

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
    pub tab_order: Vec<String>,
    pub tabs: BTreeMap<String, EditorPanel>,
    pub current_tab: String,

    #[cfg(debug_assertions)]
    pub debug: BTreeMap<String, String>,

    #[cfg(debug_assertions)]
    pub dbg_table: Table,

    pub show_settings: bool,
}


#[derive(Debug, Clone)]
pub enum EditorEvent {
    OpenFile(PathBuf),
    CloseFile(PathBuf),
    FocusFile(PathBuf),
    Ready,
    LinkClicked(Url),
    Debug(String, String),
    Edit(PathBuf, Action),
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
            last_file_change: Local::now(),
            last_interaction: Local::now(),
            tab_order: vec![],
            tabs: BTreeMap::new(),
            current_tab: "".to_string(),
            show_settings: false,
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
                    let mut editor_panel = EditorPanel::new(self.id, path.clone());
                    let mut active_modifiers = TextModifier::NONE;

                    let mut nodes: Vec<MarkdownToken> = vec![
                        MarkdownToken::new(Kind::Paragraph)
                    ];
                    let parser = pulldown_cmark::Parser::new(&text);
                    let mut current_content_string = "".to_string();
                    for event in parser {
                        match event {
                            Event::Start(tag) => {
                                match tag {
                                    Tag::Heading { level, id: _, classes: _, attrs: _ } => {
                                        nodes.push(MarkdownToken::new(Kind::Heading(level as usize)));
                                    },
                                    Tag::Paragraph => nodes.push(MarkdownToken::new(Kind::Paragraph)),
                                    Tag::Emphasis => {
                                        let mut current = nodes.last_mut().unwrap();


                                    }
                                    _ => {
                                        warn!("Tag {:?} is unknown", tag);
                                    }
                                }
                            }
                            Event::End(tag) => {
                                let mut current = nodes.last_mut().unwrap();
                                match tag {
                                    TagEnd::Heading(_) => {
                                        current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
                                        current_content_string = "".to_string();
                                    },
                                    _=> {
                                        warn!("Tag Ending {:?} is unknown", tag);
                                    }
                                }

                            }
                            Event::Text(content) => {
                                current_content_string = format!("{}{}", current_content_string, content);
                            },
                            _ => {
                                warn!("Unsupported token type: {:?}", event);
                            }
                        }
                    }
                    editor_panel.editor = text_editor::Content::with_text(&text);
                    editor_panel.preview = nodes;
                    let was_new = self.tabs.insert(path.to_str().unwrap().to_string(), editor_panel).is_none();

                    let tab_key = path.to_str().unwrap().to_string();
                    if was_new {
                        self.tab_order.push(tab_key.clone());
                    }

                    self.current_tab = tab_key.clone();
                    self.last_interaction = Local::now();
                    self.last_file_change = Local::now();
                    info!("Rendered file with the following editor data");
                    dbg!(&self);
                } else {
                    warn!("Unsupported content type found in file (not text): {}", path.display());
                }



                self.title = format!("{} - {} - Editing {}", APP_NAME, APP_VERSION, path.file_name().unwrap().to_str().unwrap());

                Task::batch([
                    #[cfg(feature = "drpc")]
                    Task::done(GlobalEvent::UpdatePresence(self.id)),
                    self.debug("workspace_name", &*self.workspace.manifest.name.clone().unwrap_or("Unknown".to_string())),
                    self.debug("workspace_path", self.workspace.manifest.parse_local_path().unwrap().to_str().unwrap()),
                    self.debug("current_tab", &self.current_tab)
                ])
            }
            EditorEvent::CloseFile(path) => {
                let id = path.to_str().unwrap().to_string();
                let index_of_tab = self.tab_order.iter().position(|k| k == &id);
                self.tab_order = self.tab_order.iter().filter(|&k| k != &id).cloned().collect();
                if let Some(tab) = self.tabs.get_mut(&id) {
                    tab.close();
                }
                self.tabs.remove(&id);
                if let Some(index) = index_of_tab {
                    if index >= self.tab_order.len() {
                        self.current_tab = self.tab_order[index-1].clone();
                    // } else if self.tab_order.len() == 0 {
                    //     // if there is no tabs available to swap to, then we need to
                    //     // show a landing page by using the internal reserved namespace
                    //     self.current_tab = "NOOT://SYSTEM/LANDING".to_string();
                    //     self.tab_order.push("NOOT://SYSTEM/LANDING".to_string());
                    //     self.tabs.insert()
                    } else {
                        self.current_tab = self.tab_order[index].clone();
                    }
                } else {
                    self.current_tab = self.tab_order[0].clone();
                }

                self.debug("current_tab", &self.current_tab)
            }
            EditorEvent::FocusFile(path) => {
                self.current_tab = path.to_str().unwrap().to_string();

                self.debug("current_tab", &self.current_tab)
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

    pub fn render_tab_bar(&self) -> Element<GlobalEvent> {
        container(
            row(
                self.tab_order.iter().map(|t| {
                    match self.tabs.get(t) {
                        Some(tab) => tab.view_tab(),
                        None => row!(text("Something went wrong")).into()
                    }
                })
            )
        ).into()
    }

    pub fn view(&self) -> Element<GlobalEvent> {
        info!("Rendering editor view.");
        info!("Active Tab: {:?}", self.current_tab);

        Container::new(
            row!(
                self.file_list.view(),
                column!(
                    self.render_tab_bar(),
                    match self.tabs.get(&self.current_tab) {
                        Some(tab) => tab.view(),
                        None => text("Loading Workspace....").color(Color::parse("#f30ad2").unwrap()).into()
                    },
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
        format!("Working on '{}'", w.tabs.get(&w.current_tab).unwrap().file.file_name().unwrap().to_str().unwrap().to_string()).leak()
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
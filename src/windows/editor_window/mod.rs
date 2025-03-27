use std::collections::BTreeMap;
use std::fs::read;
use std::path::PathBuf;
use std::sync::Arc;
use chrono::{DateTime, Local};
use discord_rich_presence::activity::{Activity, ActivityType, Timestamps};
use iced::{window, Color, Element, Size, Task, Theme};
use iced::widget::{center, column, container, mouse_area, opaque, row, scrollable, stack, text, Container};
use iced::widget::text_editor::Action;
use iced::window::{icon, Id};
use crate::app::GlobalEvent;
use crate::components::table::row::TableRow;
use crate::components::table::Table;
use crate::components::tree::TreeWidget;
use crate::consts::{APP_ICON, APP_NAME, APP_VERSION};
use crate::filesystem::config::Config;
use crate::filesystem::workspace::state::WorkspaceState;
use crate::subsystems::resolver::MediaResolver;
use crate::windows::editor_window;
use crate::windows::editor_window::editor_panel::EditorPanel;

mod editor_panel;


#[derive(Debug, Clone)]
pub enum EditorEvent {
    OpenFile(PathBuf),
    CloseFile(PathBuf),
    FocusFile(PathBuf),
    Ready,
    // LinkClicked(Url),
    Debug(String, String),
    Edit(PathBuf, Action),
    CloseSettings,
    OpenSettings
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


#[derive(Debug)]
pub struct EditorWindow {
    pub id: Id,
    pub cfg: Arc<Config>,
    pub workspace: WorkspaceState,
    pub file_list: TreeWidget,

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

    pub settings: crate::views::settings::Settings,
    pub resolver: MediaResolver,
}

impl EditorWindow {
    pub fn new(state: WorkspaceState, cfg: Arc<Config>, theme: Theme) -> (Self, Id, Task<Id>) {
        let (id, task) = window::open(Self::config());

        let mut window = Self {
            id,
            cfg: cfg.clone(),
            workspace: state.clone(),
            file_list: TreeWidget::new(state.manifest.parse_local_path().unwrap()),
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
            settings: crate::views::settings::Settings::new(cfg, state.manifest.clone()),
            resolver: MediaResolver::new()
        };

        #[cfg(debug_assertions)]
        {
            window.dbg_table = window.dbg_table.headers(TableRow::new(vec!["Field", "Value"]));
        }


        (window, id, task)
    }

    pub fn config() -> iced::window::Settings {
        iced::window::Settings {
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

    // pub fn render_doc(bytes: &[u8]) ->

    pub fn update(&mut self, event: EditorEvent) -> Task<GlobalEvent> {
        warn!("EditorEvent: {:?}", event);
        match event {
            EditorEvent::Ready => {
                let default_buffer = self.file_list.has_readme();

                // if let Some(file) = default_buffer {
                //     Task::done(GlobalEvent::Editor(self.id, EditorEvent::OpenFile(file))).chain(self.debug("Ready", "true"))
                // } else {
                let asset = self.resolver.resolve("noot://internal/test");

                if let Ok(asset) = asset {
                    let editor = EditorPanel::new_from_bytes(self.id, asset.url.clone().into(), &asset.read().unwrap());

                    self.tabs.insert(asset.url.clone(), editor);
                    self.tab_order.push("Landing Page".to_string());
                    self.current_tab = asset.url.clone();
                } else {
                    panic!("Failed to find alternative asset to load")
                }
                self.debug("Ready", "true")
                // }
            }
            EditorEvent::OpenFile(path) => {
                let buffer = read(path.clone()).unwrap();

                let text = String::from_utf8(buffer);
                if let Ok(text) = text {
                    let editor_panel = EditorPanel::new(self.id, path.clone(), text);
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
                    if self.tab_order.len() == 0 {
                        let asset = self.resolver.resolve("noot://internal/landing");

                        if let Ok(asset) = asset {
                            let editor = EditorPanel::new_from_bytes(self.id, asset.url.clone().into(), &asset.read().unwrap());

                            self.tabs.insert(asset.url.clone(), editor);
                            self.tab_order.push("Landing Page".to_string());
                            self.current_tab = asset.url.clone();
                        } else {
                            error!("Failed to find alternative asset to load");
                            error!("{:?}", asset.unwrap_err());
                            panic!("Failed to find alternative asset to load")
                        }
                    } else if index >= self.tab_order.len() {
                        self.current_tab = self.tab_order[index - 1].clone();
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
            EditorEvent::OpenSettings => {
                self.settings.show = true;
                Task::none()
            }
            EditorEvent::CloseSettings => {
                self.settings.show = false;
                Task::none()
            },

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
        let base_layer = Container::new(
            row!(
                self.file_list.view(),
                column!(
                    self.render_tab_bar(),
                    match self.tabs.get(&self.current_tab) {
                        Some(tab) => tab.view(),
                        None => text("Loading Workspace....").color(Color::parse("#f30ad2").unwrap()).into()
                    },
                )
            )
        ).into();


        if self.settings.show {
            modal(base_layer, self.settings.view(), GlobalEvent::Editor(self.id, EditorEvent::CloseSettings))
        } else {
            base_layer
        }
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
        editor_window::build_activity_from_state(&self)
    }
}

fn modal<'a, Message>(
        base: impl Into<Element<'a, Message>>,
        content: impl Into<Element<'a, Message>>,
        on_blur: Message,
    ) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        stack![
            base.into(),
            opaque(
                mouse_area(
                    center(
                        opaque(
                            content
                        )
                    ).style(|_theme| {
                        container::Style {
                            text_color: None,background: Some(
                                Color {
                                    a: 0.8,
                                    ..Color::BLACK
                                }.into()
                            ),
                            ..container::Style::default()
                        }
                    })
                ).on_press(on_blur),
            )
        ].into()
    }

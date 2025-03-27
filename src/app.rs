use std::collections::BTreeMap;
use chrono::{DateTime, Utc};
use discord_rich_presence::activity::{Activity, ActivityType, Assets, Timestamps};
use discord_rich_presence::DiscordIpcClient;
use iced::{exit, window, Element, Subscription, Task, Theme};
use iced::application::Title;
use iced::daemon::{Appearance, DefaultStyle};
use iced::widget::horizontal_space;
use iced::window::{gain_focus, Event, Id};
use crate::consts::{APP_NAME, APP_VERSION, DRPC_CLIENT_ID};
use crate::filesystem::config::Config;
use crate::filesystem::workspace::manager::MANAGER;
#[cfg(feature = "drpc")]
use crate::subsystems::discord::RPC_CLIENT;
use crate::windows::{AppWindow};
use crate::windows::build_info_window::{BuildInfoMessage, BuildInfoWindow};
use crate::windows::editor_window::{EditorEvent, EditorWindow};

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub windows: BTreeMap<Id, AppWindow>,
    has_ticked: bool,
    is_initial: bool,
    debug_window_id: Option<Id>,
    theme: Theme,
}


#[derive(Debug, Clone)]
pub enum GlobalEvent {
    Tick,
    OpenWindow(String),
    ExitApp,
    WindowClosed(Id),
    DebugMessage(String),
    DebugState(String, String),
    Editor(Id, EditorEvent),
    EditorBeam(EditorEvent),
    #[cfg(feature = "drpc")]
    UpdatePresence(Id),
}

impl App {
    pub fn new() -> (App, Task<GlobalEvent>) {
        let (config, is_initial) = Config::load_from_disk();
        let mut mgr = MANAGER.lock().unwrap();
        mgr.ingest_config(config.workspaces.clone().unwrap());

        (
            App {
                config,
                windows: BTreeMap::new(),
                has_ticked: false,
                is_initial,
                debug_window_id: None,
                theme: Theme::default(),
            },
            Task::done(GlobalEvent::Tick),
        )
    }


    pub fn update(&mut self, event: GlobalEvent) -> Task<GlobalEvent> {
        debug!("{:?}", event);
        match event {
            #[cfg(feature = "drpc")]
            GlobalEvent::UpdatePresence(id) => {
                let presence = match self.windows.get(&id) {
                    Some(window) => {
                        match window {
                            AppWindow::Editor(e) => e.get_presence(),
                            _ => {
                                Activity::new()
                                    .activity_type(ActivityType::Playing)
                                    .state("Idling")
                                    .timestamps(
                                        Timestamps::new()
                                            .start(
                                                Utc::now()
                                                    .timestamp()
                                            )
                                    )
                                    .assets(
                                        Assets::new()
                                            .large_image("bruce")
                                            .large_text(APP_NAME)
                                            .small_image("ferris")
                                            .small_text(APP_VERSION)
                                    )
                            }
                        }
                    }
                    _ => {
                        Activity::new()
                            .activity_type(ActivityType::Playing)
                            .state("Idling")
                            .timestamps(
                                Timestamps::new()
                                    .start(
                                        Utc::now()
                                            .timestamp()
                                    )
                            )
                            .assets(
                                Assets::new()
                                    .large_image("bruce")
                                    .large_text(APP_NAME)
                                    .small_image("ferris")
                                    .small_text(APP_VERSION)
                            )
                    }
                };

                RPC_CLIENT.lock().unwrap().set_activity(presence);

                Task::none()
            }

            GlobalEvent::Tick => {
                if self.has_ticked {
                    return Task::none();
                }

                self.has_ticked = true;

                #[cfg(feature = "drpc")]
                RPC_CLIENT.lock().unwrap().connect(DRPC_CLIENT_ID);

                if self.is_initial {
                    Task::done(GlobalEvent::OpenWindow("landing".to_string()))
                } else {
                    Task::done(GlobalEvent::OpenWindow("debug".to_string()))

                }
            }
            GlobalEvent::OpenWindow(name) => {
                match name.as_str() {
                    "editor" => {
                        let lo = self.config.last_open.clone().unwrap();

                        let chain = Task::done(GlobalEvent::DebugState("Workspace Loaded".to_string(), "false".to_string()))
                            .chain(Task::done(GlobalEvent::DebugState("Workspace ID".to_string(), lo.clone())));


                        let workspace = MANAGER.lock().unwrap().load_workspace(lo);
                        if let Ok(workspace) = workspace {
                            let (state, id, task) = EditorWindow::new(workspace, self.theme.clone());

                            self.windows.insert(id, AppWindow::Editor(Box::from(state)));
                            return task.discard()
                                .chain(chain)
                                .chain(Task::done(GlobalEvent::DebugState("Workspace Loaded".to_string(), "true".to_string())))
                                .chain(gain_focus(id))
                                .chain(Task::done(GlobalEvent::DebugState("Window Count".to_string(), self.windows.len().to_string())))
                                .chain(Task::done(GlobalEvent::Editor(id, EditorEvent::Ready)));
                        } else {
                            error!("Tried to open workspace editor but workspace was not ready");
                        }

                        exit().chain(chain)
                    },
                    "debug" => {
                        let (state, id, task) = BuildInfoWindow::new();
                        self.windows.insert(id, AppWindow::BuildInfo(Box::from(state)));
                        self.debug_window_id = Some(id);
                        task
                            .discard()
                            .chain(gain_focus(id))
                            .chain(Task::done(GlobalEvent::OpenWindow("editor".to_string())))
                            .chain(Task::done(GlobalEvent::DebugState("Window Count".to_string(), self.windows.len().to_string())))
                    }
                    _ => {
                        error!("Unknown window: {}", name);
                        Task::none()
                    }
                }
            }
            GlobalEvent::ExitApp => {
                let mut tasks = vec![];
                let mut ids: Vec<Id> = vec![];
                for id in self.windows.keys() {
                    ids.push(*id);
                }

                #[cfg(feature = "drpc")]
                RPC_CLIENT.lock().unwrap().disconnect();

                for id in ids {
                    let panel = self.windows.get_mut(&id).unwrap();
                    match panel {
                        AppWindow::BuildInfo(pane) => tasks.push(pane.update(BuildInfoMessage::RequestExit)),
                        //AppWindow::Editor(pane) => {} // Currently the editor window is unimplemented so cannot be closed
                        pane => {
                            error!("Failed to call RequestExit event update on window with debug ref below:");
                            error!("{:?}", pane);
                            error!("Forcing window close using runtime");
                            tasks.push(window::close(id.clone()));
                        }
                    }
                }
                Task::batch(tasks)
            }
            GlobalEvent::WindowClosed(id) => {
                debug!("WindowClosed: {:?}", id);
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    warn!("All windows now closed. Application is exiting");
                    return exit()
                }

                Task::done(GlobalEvent::DebugState("Window Count".to_string(), self.windows.len().to_string()))
            }
            GlobalEvent::DebugMessage(message) => {
                debug!("DebugMessage: {:?}", message);
                Task::none()
            }
            GlobalEvent::DebugState(name, state) => {
                if let Some(id) = self.debug_window_id {
                    if let Some(window) = self.windows.get_mut(&id) {
                        return match window {
                            AppWindow::BuildInfo(pane) => pane.update(BuildInfoMessage::SetState(name, state)),
                            _ => Task::none()
                        }
                    }
                }

                Task::none()
            }
            GlobalEvent::Editor(id, event) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    match window {
                        AppWindow::Editor(editor) => editor.update(event),
                        _ => Task::none()
                    }
                } else {
                    Task::none()
                }
            },
            GlobalEvent::EditorBeam(message) => {
                let mut task = Task::none();
                for window in self.windows.values_mut() {
                    match window {
                        AppWindow::Editor(editor) => {
                            task = task.chain(editor.update(message.clone()));
                        },
                        _ => {}
                    }
                }

                task
            }
            e => {
                error!("Unknown event: {:?}", e);
                Task::none()
            }
        }
    }

    pub fn view(&self, id: Id) -> Element<GlobalEvent> {
        match &self.windows.get(&id) {
            Some(AppWindow::Editor(editor)) => editor.view(),
            Some(AppWindow::BuildInfo(panel)) => panel.view(id),
            _ => horizontal_space().into(),
        }
    }

    pub fn title(&self, id: Id) -> String {
        match self.windows.get(&id) {
            Some(AppWindow::Editor(e)) => e.title(),
            None => String::new(),
            Some(&AppWindow::BuildInfo(_)) => format!("{} - {} - Debug Build Window", APP_NAME, APP_VERSION),
        }
    }

    pub fn theme(&self, id: Id) -> Theme {
        self.theme.clone()
    }

    pub fn scale_factor(&self, id: Id) -> f64 {
        1.0
    }

    pub fn style(&self, theme: &Theme) -> Appearance {
        theme.default_style()
    }

    pub fn subscription(&self) -> Subscription<GlobalEvent> {
        let window_events = window::events().map(|(id, e)| {
            return match e {
                Event::Closed => {
                    GlobalEvent::WindowClosed(id)
                },
                _ => GlobalEvent::DebugMessage(format!("{:?}", e)),
            }
        });
        //
        // let handle = std::sync::mpsc::channel();
        //
        //
        //
        // std::thread::spawn(move || {
        //
        // })

        Subscription::batch([
            window_events,
        ])
    }
}


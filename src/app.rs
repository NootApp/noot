use std::collections::BTreeMap;
use iced::{exit, window, Element, Subscription, Task, Theme};
use iced::daemon::{Appearance, DefaultStyle};
use iced::widget::horizontal_space;
use iced::window::{gain_focus, Event, Id};
use crate::consts::{APP_NAME, APP_VERSION};
use crate::filesystem::config::Config;
use crate::windows::{AppWindow};
use crate::windows::build_info_window::{BuildInfoMessage, BuildInfoWindow};
use crate::windows::editor_window::EditorWindow;

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub windows: BTreeMap<Id, AppWindow>,
    has_ticked: bool,
    is_initial: bool,
    debug_window_id: Option<Id>,
}


#[derive(Debug)]
pub enum GlobalEvent {
    Tick,
    OpenWindow(String),
    ExitApp,
    WindowClosed(Id),
    DebugMessage(String),
    DebugState(String, String),
}

impl App {
    pub fn new() -> (App, Task<GlobalEvent>) {
        let (config, is_initial) = Config::load_from_disk();

        (
            App {
                config,
                windows: BTreeMap::new(),
                has_ticked: false,
                is_initial,
                debug_window_id: None,
            },
            Task::done(GlobalEvent::Tick),
        )
    }




    pub fn update(&mut self, event: GlobalEvent) -> Task<GlobalEvent> {
        debug!("{:?}", event);
        match event {
            GlobalEvent::Tick => {
                if self.has_ticked {
                    return Task::none();
                }

                self.has_ticked = true;

                if self.is_initial {
                    Task::done(GlobalEvent::OpenWindow("landing".to_string()))
                } else {
                    Task::done(GlobalEvent::OpenWindow("debug".to_string()))

                }
            }
            GlobalEvent::OpenWindow(name) => {
                match name.as_str() {
                    "editor" => {
                        let (state, id, task) = EditorWindow::new();
                        self.windows.insert(id, AppWindow::Editor(Box::from(state)));
                        task.discard()
                            .chain(gain_focus(id))
                            .chain(Task::done(GlobalEvent::DebugState("Window Count".to_string(), self.windows.len().to_string())))
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
            Some(AppWindow::Editor(_)) => format!("{} - {}", APP_NAME, APP_VERSION),
            None => String::new(),
            Some(&AppWindow::BuildInfo(_)) => format!("{} - {} - Debug Build Window", APP_NAME, APP_VERSION),
        }
    }

    pub fn theme(&self, id: Id) -> Theme {
        Theme::default()
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

        Subscription::batch([
            window_events,
        ])
    }
}


use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use iced::{application, exit, Subscription};
use iced::widget::text;
use iced::window::Id;
use lazy_static::lazy_static;
use crate::config::Config;
use crate::runtime::messaging::{Message, MessageKind};
use crate::runtime::windows::{AppWindow, DesktopWindow};
use crate::runtime::windows::editor::EditorWindow;
use crate::runtime::windows::workspace::WorkspaceWindow;
use crate::storage::process::ProcessStorageManager;
use crate::storage::process::structs::setting::Setting;
use crate::storage::process::structs::workspace::Workspace;
use crate::storage::workspace::WorkspaceManager;

pub mod messaging;
pub mod windows;

// #[feature(type_alias_impl_trait)]
// pub type Task = iced::Task<impl Into<Message>>;
// #[feature(type_alias_impl_trait)]
// pub type Element<'a> = iced::Element<'a, impl Into<Message>>;


pub type Task = iced::Task<Message>;
pub type Element<'a> = iced::Element<'a, Message>;

lazy_static!(
  pub static ref GLOBAL_STATE: Arc<Mutex<AppState>> = Arc::new(
        Mutex::new(
            AppState {
                config: Config::default(),
                run_ipc: false,
                store: ProcessStorageManager::new(),
                workspaces: BTreeMap::new(),
                pid: std::process::id(),
            }
        )
    );
);



pub struct RuntimeState {
    pub windows: BTreeMap<Id, AppWindow>
}

impl RuntimeState {
    fn new() -> RuntimeState {
        RuntimeState { windows: Default::default() }
    }
}

#[derive(Debug)]
pub struct AppState {
    pub config: Config,
    pub store: ProcessStorageManager,
    pub workspaces: BTreeMap<String, Workspace>,
    pub run_ipc: bool,
    pub pid: u32,
}

pub struct Application {
    rt: RuntimeState,
    state: Arc<Mutex<AppState>>,
    open_workspace: Option<String>
}

impl Application {
    pub fn new() -> (Application, Task) {
        (Application {
            rt: RuntimeState::new(),
            state: GLOBAL_STATE.clone(),
            open_workspace: None,
        }, Message::tick().into())
    }

    pub fn title(&self, window: Id) -> String {
        let w = self.rt.windows.get(&window).unwrap();
        w.title()
    }

    pub fn update(&mut self, message: Message) -> Task {
        match message.kind {
            MessageKind::Tick => self.tick(),
            MessageKind::WindowOpen(name) => self.open_window(name),
            MessageKind::WindowMessage(wm) => {
                if let Some(id) = wm.source_id {
                    let window = self.rt.windows.get_mut(&id).unwrap();
                    window.update(wm)
                } else {
                    Task::none()
                }
            }
            MessageKind::Keybind(event) => {
                dbg!(event);
                Task::none()
            }
            MessageKind::OpenWorkspace(workspace_id) => self.open_workspace(workspace_id),
            _ => {
                info!("UnhandledMessage: {:?}", message);
                Task::none()
            }
        }
    }

    pub fn tick(&mut self) -> Task {
        // The initial first tick of the app
        let maybe_lock = self.state.try_lock();

        if maybe_lock.is_err() {
            error!("Failed to lock state");
            error!("{:?}", maybe_lock.unwrap_err());
            return exit()
        }

        let mut state = maybe_lock.unwrap();
        let workspaces = state.store.list_workspaces();


        if workspaces.is_empty() {
            // This is probably an uninitialized installation.
            // Let's show the "getting started" window.
            return Message::window_open("workspace-manager").into();


            // We need to panic, because it should not be empty
            // error!("No workspaces found");
            // error!("High probability of database corruption");
            // error!("Exiting to release database lock");
            // return exit()
        }

        for workspace in &workspaces {
            state.workspaces.insert(workspace.id.clone(), workspace.clone());
        }

        let load_last_used = state.store.get_setting::<String>("workspace.load_last").unwrap_or(Setting{key: "workspace.load_last".to_string(), value:None, enabled:false});

        if load_last_used.enabled {
            // let last_used = workspaces.last().unwrap();

            // TODO: some logic to load an editor window with the last opened workspace
        } else {
            return Message::window_open("workspace-manager").into();
        }
        Task::none()
    }

    /// Helper function for managing internal window state of the application
    pub fn open_window(&mut self, name: String) -> Task {
        match name.as_str() {
            "workspace-manager" => {
                let (context, task) = WorkspaceWindow::new();
                self.rt.windows.insert(context.id, AppWindow::WorkspaceWindow(context));
                task.discard()
            }
            "editor" => {
                let source = self.state.lock().unwrap().workspaces.get(&self.open_workspace.clone().unwrap()).cloned().unwrap();
                let mgr = WorkspaceManager::new(source).unwrap();
                let (context, task) = EditorWindow::new(mgr);
                self.rt.windows.insert(context.id, AppWindow::EditorWindow(context));

                task.discard()
            }
            _ => Task::none()
        }
    }

    pub fn open_workspace(&mut self, name: String) -> Task {
        self.open_workspace = Some(name);
        let task = self.open_window("editor".to_string());
        task
    }

    pub fn view(&self, id: Id) -> Element {
        let maybe_window = self.rt.windows.get(&id);

        if let Some(window) = maybe_window {
            window.view()
        } else {
            text(t!("coming-soon")).into()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(crate::hotkey::start)
    }
}

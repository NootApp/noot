use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use iced::{application, exit, Subscription};
use iced::widget::text;
use iced::window::Id;
use lazy_static::lazy_static;
use rusqlite::fallible_iterator::FallibleIterator;
use crate::config::Config;
use crate::runtime::messaging::{Message, MessageKind};
use crate::runtime::windows::{AppWindow, DesktopWindow};
use crate::runtime::windows::editor::EditorWindow;
use crate::runtime::windows::workspace::WorkspaceWindow;
use crate::storage::process::ProcessStorageManager;
use crate::storage::process::structs::setting::Setting;
use crate::storage::process::structs::workspace::Workspace;
use crate::storage::workspace::WorkspaceManager;

/// Holds all the message passing code for the base layer of the app. All roads lead to `crate::runtime::messaging`.
pub mod messaging;

/// Holds the definitions for each of the applications window types, and their respective internal runtimes.
pub mod windows;

// #[feature(type_alias_impl_trait)]
// pub type Task = iced::Task<impl Into<Message>>;
// #[feature(type_alias_impl_trait)]
// pub type Element<'a> = iced::Element<'a, impl Into<Message>>;


/// Globally used alias for this applications task type.
pub type Task = iced::Task<Message>;

/// Globally used alias for the element type returned by all windows when calling `.view()`.
pub type Element<'a> = iced::Element<'a, Message>;

lazy_static!(
    /// A global state instance shared across the application, used by each window to ensure that it can update the app
    /// state as is necessary. (which isn't very much)
    pub static ref GLOBAL_STATE: Arc<Mutex<AppState>> = Arc::new(
        Mutex::new(
            AppState {
                config: Config::default(),
                run_ipc: false,
                store: ProcessStorageManager::new(),
                workspaces: BTreeMap::new(),
                pid: std::process::id(),
                open_workspace: None,
            }
        )
    );
);


/// Private runtime state information, used by `crate::runtime::Application` for internal
/// logic and managing of windows and how they are rendered.
pub struct RuntimeState {
    /// A `BTreeMap` containing each window and their respective identifiers for use when rendering or updating.
    pub windows: BTreeMap<Id, AppWindow>
}

impl RuntimeState {
    /// Builds a new `RuntimeState` instance.
    fn new() -> RuntimeState {
        RuntimeState { windows: Default::default() }
    }
}

/// A global runtime state that can be modified by anyone with a copy of it at any time.
#[derive(Debug)]
pub struct AppState {
    /// The configuration data as it has been loaded from the disk.
    /// > :warning: Caution: This is currently loaded via `Default::default()` and is not persisted to disk.
    pub config: Config,

    /// The storage manager for the current application. Manages the information within the main database file.
    pub store: ProcessStorageManager,

    /// A map of the available workspaces (as found by the `ProcessStoreManager`),
    /// this contains the ID of the workspace as its key, and stores a
    /// partial workspace entry for referencing in the GUI.
    pub workspaces: BTreeMap<String, Workspace>,

    /// Whether IPC is running. (always false at this time)
    pub run_ipc: bool,

    /// The current process ID. (as assigned by the OS)
    /// This is used alongside the IPC subsystem to route messages.
    pub pid: u32,

    /// The workspace ID that should be opened when a new editor is called.
    pub open_workspace: Option<String>
}

/// The application and its primary runtime.
///
/// This handles all the logic behind running the application,
/// including message routing window management and system monitoring.
pub struct Application {
    /// Internal runtime state - Not exposed to the rest of the application.
    rt: RuntimeState,
    /// Global runtime state - Exposed to the rest of the application.
    state: Arc<Mutex<AppState>>,

    active_workspace: Arc<tokio::sync::Mutex<Option<Workspace>>>,
}

impl Application {
    /// Spawn a new Application runtime, returns a trigger task and an instance of the application to run the daemon with.
    pub fn new() -> (Application, Task) {
        (Application {
            rt: RuntimeState::new(),
            state: GLOBAL_STATE.clone(),
            active_workspace: Arc::new(Default::default()),
        }, Message::tick().into())
    }

    /// Returns the current process title for the given window, can be localised.
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
        info!("Opening window: {}", name);
        match name.as_str() {
            "workspace-manager" => {
                let (context, task) = WorkspaceWindow::new();
                self.rt.windows.insert(context.id, AppWindow::WorkspaceWindow(context));
                task.discard()
            }
            "editor" => {
                let temp_lock = self.state.lock().unwrap();
                let source = temp_lock.workspaces.get(&temp_lock.open_workspace.clone().unwrap()).cloned().unwrap();
                let mgr = WorkspaceManager::new(source.clone()).unwrap();
                let (context, task) = EditorWindow::new(mgr);
                self.rt.windows.insert(context.id, AppWindow::EditorWindow(context));

                task.discard()
            }
            _ => Task::none()
        }
    }

    pub fn open_workspace(&mut self, id: String) -> Task {
        info!("Opening workspace {}", id);
        self.state.lock().unwrap().open_workspace = Some(id);
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

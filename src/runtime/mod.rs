use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use iced::{exit, system, Subscription};
use iced::widget::text;
use iced::window::Id;
use notify_rust::{Notification, Timeout};
use lazy_static::lazy_static;
use crossbeam_queue::ArrayQueue;
use crate::config::Config;
use crate::consts::APP_NAME;
pub(crate) use crate::runtime::messaging::{Message, MessageKind};
use crate::runtime::windows::{AppWindow, DesktopWindow};
use crate::runtime::windows::editor::EditorWindow;
use crate::runtime::windows::workspace::WorkspaceWindow;
use crate::runtime::windows::splash::SplashWindow;
use crate::storage::process::ProcessStorageManager;
use crate::storage::process::structs::setting::Setting;
use crate::storage::process::structs::workspace::Workspace;
use crate::hotkey::Keybind;
use crate::runtime::workers::Job;
use crate::storage::workspace::WorkspaceManager;

/// Holds all the message passing code for the base layer of the app. All roads lead to `crate::runtime::messaging`.
pub mod messaging;

/// Holds the definitions for each of the applications window types, and their respective internal runtimes.
pub mod windows;

/// Holds the definitions for the worker thread management system
pub mod workers;

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
                skip_splash: false,
                load_workspace: None,
                queue: Arc::new(ArrayQueue::new(100))
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
    pub open_workspace: Option<String>,

    /// A mirror value of the CLI argument
    pub skip_splash: bool,

    /// A mirror value of the CLI argument
    pub load_workspace: Option<String>,

    /// Job queue
    pub queue: Arc<ArrayQueue<Job>>,
}

/// The application and its primary runtime.
///
/// This handles all the logic behind running the application,
/// including message routing window management and system monitoring.
pub struct Application {
    /// Internal runtime state - Not exposed to the rest of the application.
    pub rt: RuntimeState,
    /// Global runtime state - Exposed to the rest of the application.
    pub state: Arc<Mutex<AppState>>,

    /// The ID of the splash window (if present), only necessary because it is handled differently than other windows are
    pub splash_window: Option<Id>,
}

impl Application {
    /// Spawn a new Application runtime, returns a trigger task and an instance of the application to run the daemon with.
    pub fn new() -> (Application, Task) {
        let mut task = system::fetch_information().map(|i| Message::new(MessageKind::SysInfo(i), None)).chain(Task::done(Message::tick()));
        let skip_splash = GLOBAL_STATE.lock().unwrap().skip_splash;

        let mut app = Application {
            rt: RuntimeState::new(),
            state: GLOBAL_STATE.clone(),
            splash_window: None,
        };



        if !skip_splash {
            let splash = SplashWindow::new();
            let splash_window = splash.0;
            task = splash.1;
            app.splash_window = Some(splash_window.id.clone());
            app.rt.windows.insert(splash_window.id, AppWindow::SplashWindow(splash_window));
        }

        


        (app, task)
    }

    /// Returns the current process title for the given window, can be localised.
    pub fn title(&self, window: Id) -> String {
        let w = self.rt.windows.get(&window).unwrap();
        w.title()
    }

    pub fn theme(&self, _id: Id) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn update(&mut self, message: Message) -> Task {
        match message.kind {
            MessageKind::SysInfo(i) => {
                info!("CPU Info");
                info!("Manufacturer: {}", i.cpu_brand);
                info!("Core Count: {}", i.cpu_cores.unwrap_or_default());
                info!("");
                info!("Memory Info");
                info!("Installed: {}", i.memory_total);
                info!("Used: {}", i.memory_used.unwrap_or_default());
                info!("");
                info!("Graphics Info");
                info!("Adapter: {}", i.graphics_adapter);
                info!("Backend: {}", i.graphics_backend);
                info!("");
                info!("System Information");
                info!("Name: {}", i.system_name.unwrap_or_default());
                info!("Kernel: {}", i.system_kernel.unwrap_or_default());
                info!("Version: {}", i.system_version.unwrap_or_default());
                info!("Short Version: {}", i.system_short_version.unwrap_or_default());

                Task::none()
            }
            MessageKind::Tick => self.tick(),
            MessageKind::WindowOpen(name) => self.open_window(name),
            MessageKind::WindowMessage(wm) => {
                if let Some(id) = wm.source_id {
                    let window = self.rt.windows.get_mut(&id).unwrap();
                    window.update(wm)
                } else {
                    let splash_id = self.splash_window.clone();

                    if let Some(splash) = splash_id {
                        let window = self.rt.windows.get_mut(&splash);
                        
                        if let Some(window) = window {
                            window.update(wm)
                        } else {
                            Task::none()
                        }
                    } else {
                        Task::none()
                    }
                }
            }
            MessageKind::Keybind(event) => {
                match event {
                    Keybind::OpenLastEditor => {
                        let workspaces = self.state.lock().unwrap().store.list_workspaces();
                        let last_workspace = workspaces.first().unwrap();
                        Task::done(Message::open_workspace(last_workspace.id.clone()))
                    }
                }
            }
            MessageKind::OpenWorkspace(workspace_id) => self.open_workspace(workspace_id),
            MessageKind::WindowClose(id) => {
                let reference = self.rt.windows.get_mut(&id);

                if let Some(window) = reference {
                    info!("Attempting to close window with ID: {}", id);
                    let task = window.close();

                    self.rt.windows.remove(&id);

                    if self.rt.windows.len() == 0 {
                        warn!("Window count is 0, showing daemon notification");
                        Notification::new()
                            .summary("Background Mode")
                            .body("Noot is running in the background.\nSummon me again using alt + n")
                            .appname(APP_NAME)
                            .timeout(Timeout::Default)
                            .show().unwrap();
                    }

                    task
                } else {
                    Task::none()
                }
            }
            MessageKind::LinkOpened(url) => {
                if let Some(link) = url {
                    info!("Opening link - {}", link);
                    let _ = open::that(link);
                }

                Task::none()
            }
            MessageKind::Queue(jobs) => {
                let queue = GLOBAL_STATE.lock().unwrap().queue.clone();
                for job in jobs {
                    queue.push(job).unwrap();
                }
                Task::none()
            }
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
        //state.store.set_setting("workspace.load_last", None::<()>, true);

        if workspaces.is_empty() {
            // This is probably an uninitialized installation.
            // Let's show the "getting started" window.
            return Message::window_open("workspace-manager").into();
        }

        for workspace in &workspaces {
            state.workspaces.insert(workspace.id.clone(), workspace.clone());
        }

        let load_last_used = state.store.get_setting::<String>("workspace.load_last").unwrap_or(Setting{key: "workspace.load_last".to_string(), value:None, enabled:false});

        if let Some(wsp) = state.load_workspace.clone() {
            let sources: Vec<&Workspace> = workspaces.iter().filter(|w| w.id == wsp).collect();

            if sources.len() > 0 && sources.len() < 2 {
                // There is only one option
                let source = sources[0];
                Message::open_workspace(source.id.clone()).into()
            } else if sources.len() > 1 {
                // There are conflicting IDs, this should not 
                // be possible, but we should handle it in case
                error!("Workspace ID matched more than one workspace");
                error!("This should be impossible. But has happened anyway");
                error!("The program will now exit to protect your data");
                exit()
            } else {
                // The workspace ID didn't match any we know
                error!("Invalid workspace ID");
                exit()
            }
        } else if load_last_used.enabled {
            let last_used = workspaces.last().unwrap();
            return Message::open_workspace(last_used.id.clone()).into();
        } else {
            return Message::window_open("workspace-manager").into();
        }
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
                let mgr = WorkspaceManager::new(source.clone(), temp_lock).unwrap();
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


        let mut subscriptions: Vec<Subscription<Message>> = vec![
            iced::window::close_events().map(|id| Message::window_close(id)),
            Subscription::run(crate::hotkey::start),
            Subscription::run(workers::spawn)
        ];



        for window in self.rt.windows.values() {
            match window {
                AppWindow::SplashWindow(splash) => subscriptions.push(splash.subscribe()),
                AppWindow::EditorWindow(editor) => subscriptions.push(editor.subscribe()),
                _ => {}
            }
        }

        Subscription::batch(subscriptions)
    }
}

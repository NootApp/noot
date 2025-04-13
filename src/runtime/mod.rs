use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use iced::{exit, system, Subscription};
use iced::widget::text;
use iced::window::Id;
use notify_rust::{Notification, Timeout};
use lazy_static::lazy_static;
use crossbeam_queue::ArrayQueue;
use iced::futures::StreamExt;
use natural_tts::{Model, NaturalTtsBuilder};
use natural_tts::models::NaturalModelTrait;
use natural_tts::models::tts_rs::TtsModel;
use rusqlite::fallible_iterator::FallibleIterator;
use tokio::sync::mpsc::{channel, Sender};
use tokio::time::Instant;
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
use crate::runtime::state::AppState;
use crate::storage::workspace::WorkspaceManager;

/// Holds all the message passing code for the base layer of the app. All roads lead to `crate::runtime::messaging`.
pub mod messaging;

/// Holds the definitions for each of the applications window types, and their respective internal runtimes.
pub mod windows;

/// Holds the definitions for the worker thread management system
pub mod workers;

/// Holds the definitions for the global app state
pub mod state;

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
    pub windows: BTreeMap<Id, AppWindow>,
    pub tts: Sender<String>
}

impl RuntimeState {
    /// Builds a new `RuntimeState` instance.
    fn new(tts: Sender<String>) -> RuntimeState {
        RuntimeState {
            windows: Default::default(),
            tts
        }
    }
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
        let (tx, mut rx) = channel(1);

        std::thread::spawn(move || {
            let start = Instant::now();
            info!("Spawning TTS handling thread");
            let rt = tokio::runtime::Runtime::new().unwrap();
            let model = TtsModel::default();
            let voices = model.0.voices().unwrap();

            info!("Below are available Microsoft voices:");
            for voice in voices {
                info!("- {:?}", voice.name());
            }

            let mut tts = NaturalTtsBuilder::default()
                .default_model(Model::TTS)
                .tts_model(TtsModel::default())
                .build()
                .unwrap();

            rt.block_on(async move {
                info!("TTS thread is ready...");
                let diff = Instant::now() - start;
                info!("Started in {}ms", diff.as_millis());
                while let Some(text) = rx.recv().await {
                    info!("TTS message: '{}'", text);
                    let x = tts.say(text);
                    if x.is_err() {
                        error!("{}", x.unwrap_err());
                    }
                }
            });
        });

        let mut task = system::fetch_information().map(|i| Message::new(MessageKind::SysInfo(i), None)).chain(Task::done(Message::tick()));
        let skip_splash = GLOBAL_STATE.lock().unwrap().skip_splash;

        let mut app = Application {
            rt: RuntimeState::new(tx),
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
                info!("Version: {}", i.system_version.clone().unwrap_or_default());
                info!("Short Version: {}", i.system_short_version.unwrap_or_default());

                Task::none() //done(Message::new(MessageKind::Say(format!("This computer is running on {}", i.system_version.unwrap_or("an unknown operating system".to_string()))), None))
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
                    },
                    Keybind::ToggleDyslexia => {
                        let mut dyslexia: Setting<bool> = self.state.lock().unwrap().store.get_setting("appearance.font.dyslexic.enable").unwrap();
                        dyslexia.value = !dyslexia.value;
                        self.state.lock().unwrap().store.set_setting(dyslexia.key, dyslexia.value).unwrap();
                        Task::none()
                    }
                }
            }
            MessageKind::OpenWorkspace(workspace_id) => self.open_workspace(workspace_id),
            MessageKind::WindowClose(id) => {
                let reference = self.rt.windows.get_mut(&id);

                if let Some(window) = reference {
                    info!("Attempting to close window with ID: {}", id);
                    let mut task = window.close();
                    let run_daemon_mode: Setting<bool> = GLOBAL_STATE.lock().unwrap().store.get_setting("runtime.daemon.enable").unwrap();
                    self.rt.windows.remove(&id);

                    if self.rt.windows.len() == 0 && run_daemon_mode.value {

                        warn!("Window count is 0, showing daemon notification");
                        Notification::new()
                            .summary("Background Mode")
                            .body("Noot is running in the background.\nSummon me again using alt + n")
                            .appname(APP_NAME)
                            .timeout(Timeout::Default)
                            .show().unwrap();
                    } else if self.rt.windows.len() == 0 && !run_daemon_mode.value {
                        warn!("Window count is 0, daemon mode disabled in config, quitting");
                        task = task.chain(exit().into())
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
            MessageKind::Say(message) => {
                warn!("Saying '{}'", message);
                let outcome = tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
                    self.rt.tts.send(message).await
                });
                if outcome.is_err() {
                    error!("{}", outcome.unwrap_err());
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

        let load_last_used: Setting<bool> = state.store.get_setting("workspace.load_last").unwrap_or(Setting{key: "workspace.load_last".to_string(), value:false});

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
        } else if load_last_used.value {
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
            #[cfg(feature = "keybinds")]
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

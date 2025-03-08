//! Welcome to the Noot docs.
//! If you're reading this, congrats, you're probably more invested
//! than you should be
use iced::highlighter;
use iced::widget::{
    container, horizontal_space,
    pick_list, row, text, text_editor, tooltip,
};
use iced::window::icon;
use iced::{Size, Task, window};
use std::env;
use std::path::PathBuf;
use iced::futures::executor::block_on;
use filesystem::workspace::manager::MANAGER;

#[macro_use]
extern crate log;
use crate::events::types::Message;
use crate::filesystem::config::Config;
use crate::subsystems::discord::RPC_CLIENT;
use crate::filesystem::workspace::state::WorkspaceState;

mod build_meta;
mod components;
mod events;
mod filesystem;
mod subsystems;
mod views;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // This is definitely safe :|
    unsafe {
        env::set_var("NOOT_LOG", "debug");
    }
    pretty_env_logger::init_custom_env("NOOT_LOG");
}

#[tokio::main]
async fn main() -> iced::Result {
    // This is definitely safe :|
    unsafe {
        env::set_var("NOOT_LOG", "debug");
    }
    pretty_env_logger::init_custom_env("NOOT_LOG");

    debug!("{:?}", build_meta::VERSION);

    debug!("Starting Noot runtime");
    iced::application("Noot", Noot::update, Noot::view)
        .theme(Noot::theme)
        .window(window::Settings {
            size: Default::default(),
            position: Default::default(),
            min_size: Some(Size::new(540., 405.)),
            max_size: Some(Size::new(540., 405.)),
            visible: true,
            resizable: false,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: Some(
                icon::from_file_data(
                    include_bytes!("../static/favicon.png").as_slice(),
                    None,
                )
                .unwrap(),
            ),
            platform_specific: Default::default(),
            exit_on_close_request: true,
        })
        .font(
            include_bytes!("../static/fonts/Roboto-VariableFont_wdth,wght.ttf")
                .as_slice(),
        )
        .default_font(iced::Font {
            family: iced::font::Family::Monospace,
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        })
        .run_with(Noot::new)
}

/// The runtime struct that manages the whole app flow
#[derive(Debug)]
struct Noot<'a> {
    /// the current application theme
    theme: highlighter::Theme,

    /// the current application viewport
    viewport: ViewPort<'a>,

    /// the currently loaded configuration (if one is present)
    config: Option<Config>
}

/// This is a temporary struct used to keep the compiler happy
/// <div class="warning">
/// This is not to be used as a reference
/// </div>
#[derive(Debug)]
struct EditorWorkspace {
    pub file: Option<PathBuf>,
    pub content: text_editor::Content,
    pub theme: highlighter::Theme,
    pub word_wrap: bool,
    pub is_loading: bool,
    pub is_dirty: bool,
}

#[derive(Debug)]
enum ViewPort<'a> {
    LoadingView,
    WorkspaceView(WorkspaceState),
    LandingView(views::landing::LandingView<'a>),
    SettingsView,
}

impl<'a> Noot<'a> {
    fn new() -> (Self, Task<Message>) {
        debug!("Creating Noot runtime");
        (
            Self {
                theme: highlighter::Theme::SolarizedDark,
                viewport: ViewPort::LoadingView,
                config: None,
            },
            Task::perform(Config::load_from_disk(), Message::ConfigLoaded),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        debug!("Received message: {:?}", message);

        match message {
            Message::ConfigLoaded(cfg) => {
                let mut tasks : Vec<Task<Message>> = vec![];
                info!("Config loaded");
                self.config = Some(cfg.clone());
                let mut mgr = MANAGER.lock().unwrap();
                mgr.ingest_config(cfg.workspaces);

                let _outcome =
                    subsystems::cryptography::keys::perform_startup_checks().unwrap();



                debug!("Checking for previous workspaces");

                if let Some(prev_wsp) = cfg.last_open {
                    let load_outcome = mgr.load_workspace(prev_wsp);
                    let outcome = block_on(load_outcome);

                    tasks.push(Task::done(Message::WorkspaceLoadResult(outcome)));

                    // debug!("Previous workspace referenced, checking manifests");
                    // let workspace_manifest = cfg.workspaces.iter().filter(|p| {
                    //     debug!("Checking workspace {} ({} - {})", p.name, p.id, &prev_wsp);
                    //     if p.id == prev_wsp {
                    //         info!("Previous workspace {} ({})", p.name, prev_wsp);
                    //         return true
                    //     }
                    //     warn!("Workspace does not match");
                    //     false
                    // }).next();
                    //
                    // if let Some(workspace_manifest) = workspace_manifest {
                    //     debug!("Workspace manifest found - Attempting to load");
                    //     return Task::perform(WorkspaceState::open_workspace_from_manifest(workspace_manifest.clone()), Message::WorkspaceLoaded);
                    // } else {
                    //     warn!("Workspace manifest not found - Defaulting to LandingView");
                    // }
                } else {
                    self.viewport =
                        ViewPort::LandingView(views::landing::LandingView::new());
                }

                debug!("Checking RPC permissions");

                let mut rpc = RPC_CLIENT.lock().unwrap();

                if cfg.rpc.enable {
                    debug!("RPC is enabled in the config");
                    rpc.connect();
                } else {
                    debug!("RPC is not enabled in the config");
                    rpc.disconnect();
                }

                debug!("Config load finished...");
                return Task::batch(tasks);
            }
            Message::WorkspaceLoadResult(outcome) => {
                debug!("Workspace load event triggered");

                if let Ok(state) = outcome {
                    self.viewport = ViewPort::WorkspaceView(state);
                } else {
                    error!("Workspace load failed");
                    error!("{:?}", outcome.unwrap_err());
                }
            }
            _ => {
                warn!("Received an unknown message payload");
                dbg!(message);
            }
        }

        Task::none()
    }

    fn theme(&self) -> iced::theme::Theme {
        //if self.theme.is_dark() {
        iced::theme::Theme::TokyoNightStorm
        //} else {
        //iced::theme::Theme::Light
        //}
    }

    fn view(&self) -> iced::Element<events::types::Message> {
        match &self.viewport {
            ViewPort::LoadingView => {
                container(text("Loading... Please Wait.")).into()
            }
            ViewPort::WorkspaceView(_editor) => {
                container(text("Not Implemented")).into()
            }
            ViewPort::LandingView(view) => view.view(&self),
            ViewPort::SettingsView => container(text("Not Implemented")).into(),
        }
    }
}

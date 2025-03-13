//! Welcome to the Noot docs.
//! If you're reading this, congrats, you're probably more invested
//! than you should be
use iced::highlighter;
use iced::widget::{
    container, text, text_editor,
};
use iced::window::{icon, Id};
use iced::{Size, Task, window};
use std::env;
use std::path::PathBuf;
use hashbrown::HashMap;
use iced::futures::executor::block_on;
use filesystem::workspace::manager::MANAGER;

#[macro_use]
extern crate log;
use crate::events::types::Message;
use crate::filesystem::config::Config;
use crate::filesystem::utils::traits::{list_validation_results, Configuration};
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

    let log_level = env::var("NOOT_LOG").unwrap_or_else(|_| "info".to_uppercase());

    unsafe {
        env::set_var("NOOT_LOG", log_level.clone());
    }

    pretty_env_logger::init_custom_env("NOOT_LOG");



    #[cfg(debug_assertions)]
    if log_level != "debug" {
        warn!("Built with debug assertions, but log level is '{}'", log_level);
    }

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
    config: Option<Config>,

    windows: HashMap<String, Id>
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
                windows: HashMap::new()
            },
            Task::perform(Config::load_from_disk(), Message::ConfigLoaded),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        events::handlers::core(self, message)
    }

    fn theme(&self) -> iced::theme::Theme {
        if self.theme.is_dark() {
            iced::theme::Theme::TokyoNightStorm
        } else {
            iced::theme::Theme::Light
        }
    }

    fn view(&self) -> iced::Element<Message> {
        // debug!("Viewing window with id {}", id);
        views::render_view(self)
    }
}

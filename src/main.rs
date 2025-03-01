//! Welcome to the Noot docs.
//! If you're reading this, congrats, you're probably more invested
//! than you should be
use std::env;
use iced::{window, Size, Task};
use std::path::PathBuf;
use iced::widget::{text_editor, tooltip, row, text, button, center, self, column, container, horizontal_space, pick_list, Container};
use iced::highlighter;
use iced::window::icon;
#[macro_use]
extern crate log;
use pretty_env_logger::env_logger::Target;
use crate::events::types::Message;
use crate::filesystem::config::Config;
use crate::subsystems::discord::RPC_CLIENT;

mod events;
mod views;
mod filesystem;
mod components;
mod subsystems;
mod build_meta;


#[tokio::main]
async fn main() -> iced::Result {
    // This is definitely safe :|
    unsafe {
        env::set_var("NOOT_LOG", "debug");
    }
    pretty_env_logger::init_custom_env("NOOT_LOG");



    debug!("Starting noot runtime");
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
            icon: Some(icon::from_file_data(include_bytes!("../static/favicon.png").as_slice(), None).unwrap()),
            platform_specific: Default::default(),
            exit_on_close_request: true,
        })
        .font(include_bytes!("../static/fonts/Roboto-VariableFont_wdth,wght.ttf").as_slice())
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
}

/// This is a temporary struct used to keep the compiler happy
/// <div class="warning">
/// This is not to be used as a reference
/// </div>
#[derive(Debug)]
struct EditorWorkspace {
    file: Option<PathBuf>,
    content: text_editor::Content,
    theme: highlighter::Theme,
    word_wrap: bool,
    is_loading: bool,
    is_dirty: bool,
}

#[derive(Debug)]
enum ViewPort<'a> {
    WorkspaceView(EditorWorkspace),
    LandingView(views::landing::LandingView<'a>),
    SettingsView
}

impl <'a> Noot<'a> {
    fn new() -> (Self, Task<events::types::Message>) {
        debug!("Creating Noot runtime");
        (
            Self {
                theme: highlighter::Theme::SolarizedDark,
                viewport: ViewPort::LandingView(
                    views::landing::LandingView::new()
                ),
                config: None,
            },
            Task::perform(Config::load_from_disk(), Message::ConfigLoaded)
        )
    }

    fn update(&mut self, message: Message) -> Task<events::types::Message> {
        debug!("Received message: {:?}", message);
        match message {
            Message::ConfigLoaded(cfg) => {
                info!("Config loaded");
                self.config = Some(cfg.clone());
                let mut rpc = RPC_CLIENT.lock().unwrap();

                
                
                
                if cfg.rpc.enable {
                    rpc.connect();
                } else {
                    rpc.disconnect();
                }
            },
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
            ViewPort::WorkspaceView(editor) => {
                container(text("Not Implemented")).into()
            },
            ViewPort::LandingView(view) => {
                view.view(&self)
            }
            ViewPort::SettingsView => {
                container(text("Not Implemented")).into()
            }
        }
    }
}
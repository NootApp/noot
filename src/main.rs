//! Welcome to the Noot docs.
//! If you're reading this, congrats, you're probably more invested
//! than you should be
use hashbrown::HashMap;
use iced::{daemon, Subscription, Theme};
use iced::window::{Id, icon, Event};
use iced::{Size, Task, window};
use std::env;
use crossbeam_queue::SegQueue;
use window::Settings;
use crate::app::App;
use crate::consts::{APP_BUILD, APP_VERSION, FONT_BOLD_TTF, FONT_MEDIUM, FONT_MEDIUM_TTF, FONT_REGULAR};

#[macro_use]
extern crate log;
use crate::events::types::AppEvent;
use crate::filesystem::config::Config;
use crate::filesystem::workspace::state::WorkspaceState;
use crate::subsystems::events::{subscribe, EVENT_QUEUE};

mod build_meta;
mod components;
mod events;
mod filesystem;
mod subsystems;
mod views;
mod tray;
mod consts;
mod windows;

mod app;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // This is definitely safe :|
    unsafe {
        env::set_var("NOOT_LOG", "debug");
    }
    pretty_env_logger::init_custom_env("NOOT_LOG");
}

// #[tokio::main]
fn main() -> iced::Result {
    // This is definitely safe :|

    let log_level =
        env::var("NOOT_LOG").unwrap_or_else(|_| "info".to_uppercase());

    unsafe {
        env::set_var("NOOT_LOG", log_level.clone());
    }

    pretty_env_logger::init_custom_env("NOOT_LOG");

    #[cfg(debug_assertions)]
    if log_level != "debug" {
        warn!(
            "Built with debug assertions, but log level is '{}'",
            log_level
        );
    }

    info!("{} - {}", APP_VERSION, APP_BUILD);

    daemon(
        App::title,
        App::update,
        App::view
    )
        .font(FONT_MEDIUM_TTF)
        .font(FONT_BOLD_TTF)
        .font(FONT_REGULAR)
        .default_font(FONT_MEDIUM)
        .scale_factor(App::scale_factor)
        .style(App::style)
        .theme(App::theme)
        .antialiasing(true)
        .subscription(App::subscription)
        .run_with(App::new)


    // debug!("Starting Noot runtime");
    // iced::application("Noot", Noot::update, Noot::view)
    //     .theme(Noot::theme)
    //     .subscription(Noot::subscription)
    //     .window(Settings {
    //         size: Default::default(),
    //         position: Default::default(),
    //         min_size: Some(Size::new(540., 405.)),
    //         max_size: Some(Size::new(540., 405.)),
    //         visible: true,
    //         // fullscreen: false,
    //         // maximized: false,
    //         resizable: false,
    //         decorations: true,
    //         transparent: false,
    //         level: Default::default(),
    //         icon: Some(
    //             icon::from_file_data(
    //                 include_bytes!("../static/favicon.png").as_slice(),
    //                 None,
    //             )
    //             .unwrap(),
    //         ),
    //         platform_specific: Default::default(),
    //         exit_on_close_request: true,
    //     })
    //     .font(
    //         include_bytes!("../static/fonts/Roboto-VariableFont_wdth,wght.ttf")
    //             .as_slice(),
    //     )
    //     .default_font(iced::Font {
    //         family: iced::font::Family::Monospace,
    //         weight: iced::font::Weight::Normal,
    //         stretch: iced::font::Stretch::Normal,
    //         style: iced::font::Style::Normal,
    //     })
    //     .run_with(Noot::new)
}

/// The runtime struct that manages the whole app flow
#[derive(Debug)]
struct Noot {
    /// the current application viewport
    viewport: ViewPort,

    /// the currently loaded configuration (if one is present)
    config: Option<Config>,

    windows: HashMap<Id, Window>,
    queue: SegQueue<AppEvent>
}



#[derive(Debug)]
enum ViewPort {
    LoadingView,
    WorkspaceView(WorkspaceState),
    LandingView(views::landing::LandingView),
    SettingsView,
}

#[derive(Debug)]
enum Window {
    Splash,
    Main,
    Settings
}


impl Noot {

    fn new() -> (Self, Task<AppEvent>) {
        debug!("Creating Noot runtime");

        let mut window_map = HashMap::new();
        let id = window::get_latest();




        (
            Self {
                viewport: ViewPort::LoadingView,
                config: None,
                windows: window_map,
                queue: SegQueue::new(),
            },
            Task::none() //done(Config::load_from_disk(), AppEvent::ConfigLoaded),
        )
    }

    fn update(&mut self, message: AppEvent) -> Task<AppEvent> {
        events::handlers::core(self, message)
    }

    fn theme(&self) -> Theme {
        // if self.theme.is_dark() {
            Theme::TokyoNightStorm
        // } else {
        //     Theme::Light
        // }
    }

    fn view(&self) -> iced::Element<AppEvent> {
        // debug!("Viewing window with id {}", id);
        views::render_view(self)
    }

    fn subscription(&self) -> Subscription<AppEvent> {
        error!("Subscribing to noot runtime");

        let window_sub = window::events().map(|(id, e)| {
           match e {
               Event::Opened {..} => AppEvent::WindowOpened(id),
               Event::Closed => AppEvent::WindowClosed(id),
               Event::Moved(position) => AppEvent::WindowMoved(id, position),
               Event::Resized(dimensions) => AppEvent::WindowResized(id, dimensions),
               Event::CloseRequested => AppEvent::WindowCloseRequested(id),
               Event::Focused => AppEvent::WindowFocused(id),
               Event::Unfocused => AppEvent::WindowUnfocused(id),
               Event::FileHovered(file) => AppEvent::WindowFileHovered(id, file),
               Event::FileDropped(file) => AppEvent::WindowFileDropped(id, file),
               Event::FilesHoveredLeft => AppEvent::WindowFilesHoveredLeft(id),
               _ => AppEvent::Ignored
           }
        });

        Subscription::batch(
            vec![
                window_sub,
                Subscription::run(subscribe)
            ]
        )

    }
    //
    // fn open_window(&mut self, kind: Window, settings: window::Settings) {
    //     let (id, task) = window::open(settings);
    //     self.windows.insert(id, kind);
    //
    // }
}
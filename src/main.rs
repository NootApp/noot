use std::env;
use iced::daemon;
use log::info;
use rust_i18n::set_locale;
use sys_locale::get_locale;
use clap::Parser;
use crate::runtime::{Application, GLOBAL_STATE};
use crate::consts::*;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nanoid;
#[macro_use]
extern crate rust_i18n;
#[macro_use]
extern crate bitflags;


i18n!("locales", fallback = "en");

/// # Noot
/// An experimental markdown desktop editor.


/// The testing framework harness entrypoint.
#[cfg(test)]
#[ctor::ctor]
pub fn init() {
    // This is definitely safe :|
    unsafe {
        env::set_var("NOOT_LOG", "debug");
    }
    pretty_env_logger::init_custom_env("NOOT_LOG");
}

/// Application entrypoint.
pub fn main() -> iced::Result {
    let mut args = cli::Args::parse();

    args.process();

    // if args.load_workspace.is_some() {
        args.skip_splash = true; // Disable the splash screen for now.
    // }

    GLOBAL_STATE.lock().unwrap().skip_splash = args.skip_splash;
    GLOBAL_STATE.lock().unwrap().load_workspace = args.load_workspace;

    // This is definitely safe :|
    let log_level =
        env::var("NOOT_LOG").unwrap_or_else(|_| "info".to_uppercase());

    unsafe {
        env::set_var("NOOT_LOG", format!("{},iced=off", log_level));
    }
    
    pretty_env_logger::init_custom_env("NOOT_LOG");

    #[cfg(debug_assertions)]
    if log_level != "debug" {
        warn!(
            "Built with debug assertions, but log level is '{}'",
            log_level
        );
    }


    info!("{} - {}", APP_NAME, APP_VERSION);
    info!("Compiled with features:");
    #[cfg(feature = "ipc")]
    info!("- IPC");
    #[cfg(feature = "rich-presence")]
    info!("- Rich Presence");
    #[cfg(feature = "i18n")]
    info!("- I18N");
    #[cfg(feature = "plugins")]
    info!("- Plugins");

    info!("");
    info!("Compiled with languages");
    let locales = available_locales!();
    for locale in locales {
        info!("- {}", locale);
    }


    info!("Getting system locale");
    let locale = get_locale().unwrap_or_else(|| "en-US".to_string());
    info!("System locale: {}", locale);


    let config_locale = GLOBAL_STATE.lock().unwrap().store.get_setting::<String>("language.locale");

    if let Some(user_locale) = config_locale {
        let upl = user_locale.value.clone();
        info!("User requested locale: {}", upl);
        info!("Updating locale");
        set_locale(&upl);
    } else {
        info!("User locale not configured - using system default");
        set_locale(&locale);
        info!("Setting default locale");
    }

    daemon(
        Application::title,
        Application::update,
        Application::view
    )
        .font(FONT_MONOSPACE)
        .font(FONT_REGULAR)
        .font(FONT_MEDIUM_TTF)
        .font(FONT_BOLD_TTF)
        .font(material_icons::FONT)
        .theme(Application::theme)
        .subscription(Application::subscription)
        .run_with(Application::new)
}


/// Constant values used frequently within the application.
pub mod consts;

/// The runtime core of the app.
pub mod runtime;

/// The configuration management section.
pub mod config;

/// The security management section.
pub mod security;

/// The asset management section.
pub mod assets;

/// The storage management section.
pub mod storage;

/// Utility functions available throughout the app.
pub mod utils;

/// The hotkey management section.
#[cfg(feature = "keybinds")]
pub mod hotkey;

/// The inter-process communication management section.
#[cfg(feature = "ipc")]
pub mod ipc;

/// Command-line arguments
pub mod cli;

/// Plugin API management section.
#[cfg(feature = "plugins")]
pub mod plugins;

/// Theme engine section
pub mod ui;
use std::env;
use iced::daemon;
use log::info;
use rust_i18n::set_locale;
use sys_locale::get_locale;
use consts::*;
use crate::runtime::{Application, GLOBAL_STATE};

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nanoid;
#[macro_use]
extern crate rust_i18n;


i18n!("locales", fallback = "en");


#[cfg(test)]
#[ctor::ctor]
fn init() {
    // This is definitely safe :|
    unsafe {
        env::set_var("NOOT_LOG", "debug");
    }
    pretty_env_logger::init_custom_env("NOOT_LOG");
}

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


    info!("{} - {}", APP_NAME, APP_VERSION);
    info!("Compiled with features:");
    #[cfg(feature = "ipc")]
    info!("- IPC");
    #[cfg(feature = "rich-presence")]
    info!("- Rich Presence");
    #[cfg(feature = "i18n")]
    info!("- I18N");

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
        if user_locale.value.is_some() {
            let upl = user_locale.value.clone().unwrap();
            info!("User requested locale: {}", upl);
            info!("Updating locale");
            set_locale(&upl);
        } else {
            GLOBAL_STATE.lock().unwrap().store.set_setting("language.locale", locale.clone(), true);

            info!("User locale is nullified - using system default");
            set_locale(&locale);
            info!("Setting default locale");
        }
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
        .subscription(Application::subscription)
        .run_with(Application::new)
}

pub mod consts;
pub mod runtime;
pub mod config;
pub mod security;
pub mod assets;
pub mod storage;
pub mod utils;

pub mod hotkey;

#[cfg(feature = "ipc")]
pub mod ipc;
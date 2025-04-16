use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use lightningcss::{
    bundler::{Bundler, FileProvider},
    stylesheet::ParserOptions
};
use lightningcss::declaration::DeclarationBlock;
use lightningcss::printer::PrinterOptions;
use lightningcss::selector::Selector;
use lightningcss::stylesheet::StyleSheet;
use cssparser::Parser;

#[derive(Debug)]
pub struct ThemeManager<'t> {
    pub active: &'t str,
    pub themes: BTreeMap<&'t str, Theme<'t>>
}

impl <'t> ThemeManager<'t> {
    pub fn new() -> ThemeManager<'t> {
        let mut tm = ThemeManager {
            active:  match dark_light::detect().unwrap_or(dark_light::Mode::Light) {
                dark_light::Mode::Dark => "Noot Dark",
                dark_light::Mode::Light => "Noot Light",
                dark_light::Mode::Unspecified => "Noot Light",
            },
            themes: BTreeMap::new()
        };

        // Load default built in themes
        let day = Theme::from_bytes("Noot Dark", include_bytes!("../../../themes/compiled/light.css"));
        let night = Theme::from_bytes("Noot Dark", include_bytes!("../../../themes/compiled/dark.css"));

        tm.add_theme(day);
        tm.add_theme(night);

        tm
    }

    pub fn add_theme(&mut self, theme: Theme<'t>) {
        info!("Importing theme '{}'", theme.name);
        self.themes.insert(theme.name, theme);
    }
}

#[derive(Debug)]
pub struct Theme<'t> {
    name: &'t str,
    internal: StyleSheet<'t, 't>
}



impl Theme<'_> {
    pub fn from_css(name: &str, file: PathBuf) -> Theme {
        Theme {
            name,
            internal: StyleSheet::parse(Bundler::new(&FileProvider::new(), None, ParserOptions::default()).bundle(file.as_path()).unwrap().to_css(PrinterOptions::default()).unwrap().code.leak(), ParserOptions::default()).unwrap(),
        }
    }

    pub fn from_bytes<'a>(name: &'a str, data: &'a [u8]) -> Theme<'a> {
        Theme {
            name,
            internal: StyleSheet::parse(String::from_utf8_lossy(data).to_string().leak(), ParserOptions::default()).unwrap(),
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn evaluate<S: Into<String>>(&self, selector: S) {
        let p = Parser::new()
        let selector = Selector::parse(&Parser::new(), selector).unwrap();
        dbg!(&self.internal.rules);
    }
}

lazy_static!(
    pub static ref THEMES: Mutex<ThemeManager<'static>> = Mutex::new(ThemeManager::new());
);
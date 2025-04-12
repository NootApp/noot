use dark_light::Mode;
use serde_derive::{Deserialize, Serialize};

/// A struct representing the appearance settings for the app
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct AppearanceSettings {
    /// The main font used for text in the app
    pub primary_font: String,
    /// The font used for code blocks and monospaced text within the app
    pub monospace_font: String,

    /// The name of the theme to be used when rendering the app
    pub theme: ThemeSettings,

    /// The default text size of standard text.
    /// All other text is a ratio of this
    pub text_size: f32,

    /// Dyslexia mode makes all text sans-serif
    pub dyslexia_mode: bool,

    /// Dyslexia font
    pub dyslexia_font: String,

    /// Monospaced Dyslexia font
    pub dyslexia_mono_font: String,

    /// Color blindness settings
    pub color_blind_mode: ColorBlindModes,

    /// TTS Settings
    pub text_to_speech: bool,

    /// TTS Provider,
    pub tts_provider: TTSProvider
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum TTSProvider {
    None,
}

pub enum GTTS {

}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum ColorBlindModes {
    Off,
    Protanopia,
    Deuteranopia,
    Tritanopia,
    Achromatopsia,
    AnomalousTrichromacy,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct ThemeSettings {
    pub name: String,
    pub variant: ThemeVariant,
    pub day_night_cycle: bool,
    pub day_variant: ThemeVariant,
    pub night_variant: ThemeVariant,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum ThemeVariant {
    Light,
    Dark,
    Custom(String)
}

// impl AppearanceSettings {
//     pub fn new() {
//
//     }
// }

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            primary_font: "Roboto".to_string(),
            monospace_font: "Roboto Mono".to_string(),
            theme: ThemeSettings::default(),
            text_size: 16.,
            dyslexia_mode: false,
            dyslexia_font: "OpenDyslexic".to_string(),
            dyslexia_mono_font: "OpenDyslexic Mono".to_string(),
            color_blind_mode: ColorBlindModes::Off,
            text_to_speech: false,
            tts_provider: TTSProvider::None,
        }
    }
}

impl Default for ThemeSettings {
    fn default() -> Self {
        let system_theme = dark_light::detect().unwrap();
        Self {
            name: "".to_string(),
            variant: match system_theme {
                Mode::Dark => ThemeVariant::Dark,
                Mode::Light => ThemeVariant::Light,
                Mode::Unspecified => ThemeVariant::Light
            },
            day_night_cycle: false,
            day_variant: ThemeVariant::Light,
            night_variant: ThemeVariant::Dark,
        }
    }
}
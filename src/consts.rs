use iced::Font;
use git_version::git_version;


// Build Metadata
pub const APP_NAME: &str = "Noot";

pub const APP_BUILD: &str = git_version!(
    prefix = "git:",
    cargo_prefix = "cargo:",
    fallback = "unknown"
);

pub const APP_VERSION: &str = "0.1.0";



// Fonts
pub const FONT_NAME: &str = "Roboto";
pub const FONT_NAME_MONO: &str = "Roboto Mono";

pub const FONT_MONOSPACE: &[u8] = include_bytes!("../static/fonts/RobotoMono-Medium.ttf");

pub const FONT_REGULAR: &[u8] = include_bytes!("../static/fonts/Roboto-Regular.ttf");

pub const FONT_MEDIUM_TTF: &[u8] = include_bytes!("../static/fonts/Roboto-Medium.ttf");

pub const FONT_BOLD_TTF: &[u8] = include_bytes!("../static/fonts/Roboto-Bold.ttf");

lazy_static!(
    pub static ref FONTS: Vec<&'static [u8]> = vec![
        include_bytes!("../static/fonts/NotoSansLiving-Regular.ttf"),
        include_bytes!("../static/fonts/OpenDyslexic3-Bold.ttf"),
        include_bytes!("../static/fonts/OpenDyslexic3-Regular.ttf"),
        include_bytes!("../static/fonts/OpenDyslexicMono-Regular.otf"),
        include_bytes!("../static/fonts/Roboto-Medium.ttf"),
        include_bytes!("../static/fonts/Roboto-Regular.ttf"),
        include_bytes!("../static/fonts/Roboto-Bold.ttf"),
        include_bytes!("../static/fonts/RobotoMono-Medium.ttf"),
        // include_bytes!("../static/fonts/Roboto-Light.ttf"),
        // include_bytes!("../static/fonts/Roboto-LightItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto-MediumItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto-SemiBold.ttf"),
        // include_bytes!("../static/fonts/Roboto-SemiBoldItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto-Thin.ttf"),
        // include_bytes!("../static/fonts/Roboto-ThinItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto-VariableFont_wdth,wght.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-Black.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-BlackItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-Bold.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-BoldItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-ExtraBold.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-ExtraBoldItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-ExtraLight.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-ExtraLightItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-Italic.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-Light.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-LightItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-Medium.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-MediumItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-Regular.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-SemiBold.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-SemiBoldItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-Thin.ttf"),
        // include_bytes!("../static/fonts/Roboto_Condensed-ThinItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-Black.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-BlackItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-Bold.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-BoldItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-ExtraBold.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-ExtraBoldItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-ExtraLight.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-ExtraLightItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-Italic.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-Light.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-LightItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-Medium.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-MediumItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-Regular.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-SemiBold.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-SemiBoldItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-Thin.ttf"),
        // include_bytes!("../static/fonts/Roboto_SemiCondensed-ThinItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto-Black.ttf"),
        // include_bytes!("../static/fonts/Roboto-BlackItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto-BoldItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto-ExtraBold.ttf"),
        // include_bytes!("../static/fonts/Roboto-ExtraBoldItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto-ExtraLight.ttf"),
        // include_bytes!("../static/fonts/Roboto-ExtraLightItalic.ttf"),
        // include_bytes!("../static/fonts/Roboto-Italic.ttf"),

    ];
);


pub const FONT_MEDIUM: Font = Font {
    family: iced::font::Family::Name(FONT_NAME),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_MONO: Font = Font {
    family: iced::font::Family::Name(FONT_NAME_MONO),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,

};

pub const FONT_BOLD: Font = Font {
    family: iced::font::Family::Name(FONT_NAME),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const FONT_BOLD_ITALIC: Font = Font {
    family: iced::font::Family::Name(FONT_NAME),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Italic,
};

pub const FONT_ITALIC: Font = Font {
    family: iced::font::Family::Name(FONT_NAME),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Italic,
};

pub const FONT_ICON: Font = Font {
    family: iced::font::Family::Name("Material Icons"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

// Icons
pub const APP_ICON: &[u8] = include_bytes!("../static/favicon.png");


// Markdown constants

pub const HEADER_SIZE_1: f32 = 32.;
pub const HEADER_SIZE_2: f32 = 28.;
pub const HEADER_SIZE_3: f32 = 22.;
pub const HEADER_SIZE_4: f32 = 18.;
pub const HEADER_SIZE_5: f32 = 14.;
pub const TEXT_SIZE: f32 = 12.;


// Window Level Constants
// Splash Window
pub const SPLASH_ART: &[u8] = include_bytes!("../static/splash.gif");



// Colors

pub const BUTTON_DEFAULT_BACKGROUND: i32 = 0x233fa0;
pub const BUTTON_CONFIRM_BACKGROUND: i32 = 0x355E3B;
pub const BUTTON_DANGER_BACKGROUND: i32 = 0xc34a30;
pub const BUTTON_DEFAULT_TEXT: i32 = 0xfafafa;
// pub const BUTTON_CONFIRM_TEXT: i32 = 0x


pub const TEXT_INPUT_INVALID: i32 = BUTTON_DANGER_BACKGROUND;
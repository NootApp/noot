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

pub const FONT_REGULAR: &[u8] = include_bytes!("../static/fonts/Roboto-Regular.ttf");

pub const FONT_MEDIUM_TTF: &[u8] = include_bytes!("../static/fonts/Roboto-Medium.ttf");

pub const FONT_BOLD_TTF: &[u8] = include_bytes!("../static/fonts/Roboto-Bold.ttf");

pub const FONT_MEDIUM: Font = Font {
    family: iced::font::Family::Name(FONT_NAME),
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

// Icons
pub const APP_ICON: &[u8] = include_bytes!("../static/favicon.png");

#[cfg(feature = "drpc")]
pub const DRPC_CLIENT_ID: &'static str = include_str!("../client_id");
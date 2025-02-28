use iced::{window, Size, Task};
use std::path::PathBuf;
use iced::widget::{text_editor, tooltip, row, text, button, center, self, column, container, horizontal_space, pick_list, Container};
use iced::highlighter;
use iced::window::icon;

mod events;
mod views;

mod components;

#[tokio::main]
async fn main() -> iced::Result {
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



#[derive(Debug)]
struct Noot<'a> {
    theme: highlighter::Theme,
    viewport: ViewPort<'a>
}


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
        (
            Self {
                theme: highlighter::Theme::SolarizedDark,
                viewport: ViewPort::LandingView(
                    views::landing::LandingView::new()
                )
            },
            Task::none()
        )
    }

    fn update(&mut self, message: events::types::Message) -> Task<events::types::Message> {
        match message {
            _ => {
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


// fn action<'a, Message: Clone + 'a>(
//     content: impl Into<iced::Element<'a, Message>>,
//     label: &'a str,
//     on_press: Option<Message>,
// ) -> iced::Element<'a, Message> {
//     let action = button(center(content).width(30));
//
//     if let Some(on_press) = on_press {
//         tooltip(
//             action.on_press(on_press),
//             label,
//             tooltip::Position::FollowCursor,
//         )
//             .style(container::rounded_box)
//             .into()
//     } else {
//         action.style(button::secondary).into()
//     }
// }

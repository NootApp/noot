use iced::Task;
use std::path::PathBuf;
use iced::widget::{text_editor, tooltip, row, text, button, center, self, column, container, horizontal_space, pick_list};
use iced::highlighter;

mod events;

#[tokio::main]
async fn main() -> iced::Result {
    iced::application("Noot", Noot::update, Noot::view)
        .theme(Noot::theme)
        .font(include_bytes!("../static/fonts/NotoSansLiving-Regular.ttf").as_slice())
        .default_font(iced::Font::MONOSPACE)
        .run_with(Noot::new)
}



#[derive(Debug)]
struct Noot {
    theme: highlighter::Theme,
    viewport: ViewPort
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
enum ViewPort {
    WorkspaceView(EditorWorkspace),
    LandingView,
    SettingsView
}

impl Noot {
    fn new() -> (Self, Task<events::types::Message>) {
        (
            Self {
                theme: highlighter::Theme::SolarizedDark,
                viewport: ViewPort::LandingView
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
        column![
            center(
                column![
                        row![
                            center(button(center(text("Noot Noot!")).width(120)))
                        ].padding(iced::Padding {
                                top: 5.,
                                left: 10.,
                                right: 10.,
                                bottom: 5.
                            })
                            .height(50),
                    row![
                        column![
                            button(center(text("New Project"))),
                            center(text("CTRL + N"))
                        ],
                        column![
                            button(center(text("Open Folder"))),
                            center(text("CTRL + O"))
                        ],
                        column![
                            button(center(text("Cloud Project"))),
                            center(text("CTRL + C"))
                        ]
                    ]
                        .spacing(10)
                        .width(400)
                        .padding(iced::Padding {
                            top: 5.,
                            left: 10.,
                            right: 10.,
                            bottom: 5.
                        })
                        .height(80),
                ]
            )

        ].into()
    }
}


fn action<'a, Message: Clone + 'a>(
    content: impl Into<iced::Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> iced::Element<'a, Message> {
    let action = button(center(content).width(30));

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
            .style(container::rounded_box)
            .into()
    } else {
        action.style(button::secondary).into()
    }
}

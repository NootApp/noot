use crate::Noot;
use crate::events::types::AppEvent;
use crate::views::landing::new_workspace::NewWorkspaceView;
use iced::Background::Gradient;
use iced::Element;
use iced::gradient::Linear;
use iced::widget::container::Style;
use iced::widget::{Container, button, center, column, container, row, text};
use iced::{Bottom, Length, Padding, Theme, color};

mod cloud_sync;
mod new_workspace;
mod open_workspace;

#[derive(Debug, Clone)]
pub struct LandingView {
    subview: SubView,
}

#[derive(Debug, Clone)]
pub enum SubView {
    None,
    NewWorkspace(NewWorkspaceView),
    OpenWorkspace(),
    CloudWorkspace(),
}

impl LandingView {
    pub fn new() -> LandingView {
        LandingView {
            // subview: SubView::None,
            subview: SubView::NewWorkspace(NewWorkspaceView::new()),
        }
    }

    pub(crate) fn view(&self, _parent: &Noot) -> Element<AppEvent> {
        match &self.subview {
            SubView::None => Container::new(column![center(
                column![
                    row![center(column![
                        center(
                            text("Noot Noot!")
                                .font(iced::Font {
                                    family: iced::font::Family::Monospace,
                                    weight: iced::font::Weight::ExtraBold,
                                    stretch: iced::font::Stretch::Normal,
                                    style: iced::font::Style::Normal,
                                })
                                .size(32)
                                .color(color!(0xfff1aa))
                        ),
                        center(
                            text("Welcome to the future of note taking")
                                .width(350.)
                        )
                    ])]
                    .padding(Padding {
                        top: 5.,
                        left: 10.,
                        right: 10.,
                        bottom: 5.
                    })
                    .align_y(Bottom)
                    .height(100),
                    row![center(
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
                        .spacing(10.)
                        .width(475.)
                        .padding(Padding {
                            top: 5.,
                            left: 10.,
                            right: 10.,
                            bottom: 5.
                        })
                        .height(80)
                    )]
                ]
                .height(170.)
            )])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(gradient_background)
            .into(),
            SubView::NewWorkspace(workspace) => {
                Container::new(workspace.view(self))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(gradient_background)
                    .padding(Padding::from(5))
                    .into()
            }
            _ => container(text("Not Implemented")).into(),
        }
    }
}

fn gradient_background(_theme: &Theme) -> Style {
    Style {
        text_color: None,
        background: Some(Gradient(
            Linear::new(iced::Radians(0.785))
                .add_stop(0.0, color!(0x116731))
                .add_stop(0.15, color!(0x04050b))
                .add_stop(0.85, color!(0x04050b))
                .add_stop(1.0, color!(0x116731))
                .into(),
        )),
        border: Default::default(),
        shadow: Default::default(),
    }
}

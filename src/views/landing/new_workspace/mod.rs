use crate::components::form::text_input::TextInput;
use crate::events::types::AppEvent;
use crate::views::landing::LandingView;
use iced::Background::Color;
use iced::alignment::Horizontal;
use iced::border::Radius;
use iced::widget::container::Style;
use iced::widget::{column, container, row, scrollable};
use iced::{Border, Element, color};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct NewWorkspaceView {
    pub workspace_name: Option<String>,
    pub workspace_path: Option<PathBuf>,
    pub parent_folder_exists: bool,
    pub enable_plugins: bool,
    pub template_id: Option<String>,

    pub workspace_name_input: TextInput,
}

impl<'a> NewWorkspaceView {
    pub fn new() -> Self {
        Self {
            workspace_name: None,
            workspace_path: None,
            parent_folder_exists: false,
            enable_plugins: false,
            template_id: None,
            workspace_name_input: TextInput::new("Workspace Name", "", false),
        }
    }

    pub fn view(&self, _content: &LandingView) -> Element<AppEvent> {
        container(scrollable(column![row![self.workspace_name_input.view()]]))
            .height(395)
            .width(530)
            .style(move |_: &_| Style {
                background: Some(Color(color!(0x1a1a1a))),
                border: Border {
                    radius: Radius::new(5.),
                    color: iced::Color::TRANSPARENT,
                    width: 0.,
                },
                ..Default::default()
            })
            .align_x(Horizontal::Center)
            .into()
    }
}

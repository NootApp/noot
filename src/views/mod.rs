use crate::events::types::AppEvent;
use crate::{Noot, ViewPort};
use iced::widget::{container, text};

pub mod editor;
pub mod landing;

pub fn render_view(noot: &Noot) -> iced::Element<AppEvent> {
    match &noot.viewport {
        ViewPort::LoadingView => {
            container(text("Loading... Please Wait.")).into()
        }
        ViewPort::WorkspaceView(editor) => editor::render(editor),
        ViewPort::LandingView(view) => view.view(noot),
        ViewPort::SettingsView => container(text("Not Implemented")).into(),
    }
}


pub mod settings;
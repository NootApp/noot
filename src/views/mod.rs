use iced::widget::{container, text};
use crate::events::types::Message;
use crate::{Noot, ViewPort};

pub mod landing;




pub fn render_view<'a>(noot: &'a Noot<'_>) -> iced::Element<'a, Message> {
    match &noot.viewport {
        ViewPort::LoadingView => {
            container(text("Loading... Please Wait.")).into()
        }
        ViewPort::WorkspaceView(editor) => {
            dbg!(&editor);
            container(text("Not Implemented")).into()
        }
        ViewPort::LandingView(view) => view.view(noot),
        ViewPort::SettingsView => container(text("Not Implemented")).into(),
    }
}
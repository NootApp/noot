use iced::Element;
use iced::widget::{column, container, text};
use crate::Noot;
use crate::events::types::Message;
use crate::filesystem::workspace::manager::MANAGER;
use crate::filesystem::workspace::state::{Screen, WorkspaceState};

pub fn render(editor: &WorkspaceState) -> Element<Message> {
    dbg!(&editor);
    match editor.viewport {
        Screen::Welcome => render_welcome_screen(),
        Screen::Empty => render_empty_screen(),
        _ => text!("Not Implemented yet").into()
    }
}



fn render_welcome_screen<'a>() -> iced::Element<'a, Message> {
    container(
        column!(
            text!("Welcome to Noot"),
            text!("A new way to keep notes.")
        )
    ).into()
}

fn render_empty_screen<'a>() -> iced::Element<'a, Message> {
    container(
        column!(
            text!("Empty screen"),
        )
    ).into()
}
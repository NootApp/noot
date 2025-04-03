use iced::Subscription;
use crate::runtime::{Element, Task};
use crate::runtime::messaging::Message;
use crate::runtime::windows::editor::messaging::EditorMessage;


pub trait StatusBarWidget {
    fn update(&mut self, message: EditorMessage) -> Task;
    fn render(&self) -> Element;
    fn subscribe(&self) -> Subscription<Message>;
}


///// A clock widget for the status bar
//pub mod clock;

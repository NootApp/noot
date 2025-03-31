use crate::runtime::Element;
use iced::Subscription;
use crate::runtime::messaging::Message;

pub trait StatusBarWidgetMessage: Into<Message> {
    type Id;
}


pub trait StatusBarWidget {
    fn update<m: StatusBarWidgetMessage>(&mut self, message: m) -> crate::runtime::Task;
    fn render(&self) -> Element;
    fn subscribe(&self) -> Subscription<Message>;
}


/// A clock widget for the status bar
pub mod clock;

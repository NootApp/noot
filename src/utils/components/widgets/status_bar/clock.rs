use chrono::{Local, DateTime};
use iced::Subscription;
use iced::widget::text;
use super::StatusBarWidget;
use crate::runtime::windows::editor::messaging::EditorMessage;
use crate::runtime::{Task, Element};
use crate::runtime::messaging::Message;

pub enum ClockFace {
    Hours12,
    Hours24,
}

pub enum ClockMessage {
    Tick,
}

pub struct ClockWidget {
    mode: ClockFace,
    flash: bool,
    show_seconds: bool,
    time: DateTime<Local>
}


impl ClockWidget {
    pub fn new() -> Self {
        Self {
            mode: ClockFace::Hours24,
            flash: false,
            show_seconds: false,
            time: Local::now(),
        }
    }
}


impl StatusBarWidget for ClockWidget {
    fn update(&mut self, _message: EditorMessage) -> Task {
        Task::none()
    }

    fn render(&self) -> Element {
        text("Test Clock").into()
    }

    fn subscribe(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

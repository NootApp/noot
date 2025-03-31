use iced::{window, Length, Size, Theme, Task as IcedTask};
use iced::widget::container;
use iced::window::{icon, Icon, Id, Level, Position, Settings};
use iced_gif::{gif, Frames, Gif};
use crate::consts::{APP_ICON, APP_NAME, APP_VERSION, SPLASH_ART};
use crate::runtime::messaging::{Message, WindowMessageKind};
use crate::runtime::{Element, Task};
use crate::runtime::windows::DesktopWindow;

#[derive(Debug)]
pub struct SplashWindow {
    id: Id,
    frames: Frames,
}

impl SplashWindow {
    fn new() -> (Self, IcedTask<Id>) {
        let (id, task) = window::open(Self::settings());
        let frames = Frames::from_bytes(SPLASH_ART.into());

        if frames.is_err() {
            error!("SplashWindow::new: Failed to decode gif");
            error!("Passthrough: {:?}", frames.err().unwrap());
            panic!("SplashWindow::new: Failed to decode gif");
        }

        (Self { id, frames: frames.unwrap() }, task)
    }
}


impl DesktopWindow<SplashWindow, Message, Message> for SplashWindow {


    fn settings() -> Settings {
        Settings {
            size: Size::new(335.,298.),
            position: Position::Centered,
            min_size: None,
            max_size: None,
            visible: true,
            resizable: false,
            decorations: false,
            transparent: true,
            level: Level::Normal,
            icon: Some(icon::from_file_data(APP_ICON, None).unwrap()),
            platform_specific: Default::default(),
            exit_on_close_request: false,
        }
    }

    fn title(&self) -> String {
        format!("{} - {} - Starting Up", APP_NAME, APP_VERSION)
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }

    fn update(&mut self, _: Message) -> Task {
        Task::none()
    }

    fn view(&self) -> Element {
        container(gif(&self.frames))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
    
    fn close(&mut self) -> Task {
        window::close(self.id).chain(Task::done(Message::window_close(self.id)))
    }
}
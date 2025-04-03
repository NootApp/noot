use iced::{window, Length, Size, Theme, Task as IcedTask};
use iced::widget::{container, stack, row, center, text, progress_bar};
use iced::Subscription;
use iced::time::{self, Duration};
use iced::window::{icon, Id, Level, Position, Settings};
use iced_gif::{gif, Frames};
use crate::consts::{APP_ICON, APP_NAME, APP_VERSION, SPLASH_ART};
use crate::runtime::messaging::{Message, WindowMessage, WindowMessageKind};
use crate::runtime::{Element, Task};
use crate::runtime::windows::DesktopWindow;

#[derive(Debug, Clone)]
pub enum SplashWindowMessageKind {
    StatusMessage(String),
    SplashSurvived
}

#[derive(Debug, Clone)]
pub struct SplashWindowMessage {
    kind: SplashWindowMessageKind,
}


impl SplashWindowMessage {
    pub fn new(kind: SplashWindowMessageKind) -> Self {
        Self {
            kind,
        }
    }

    pub fn splash_survived() -> Self {
        Self::new(SplashWindowMessageKind::SplashSurvived)
    }
}

impl Into<SplashWindowMessage> for WindowMessageKind {
    fn into(self) -> SplashWindowMessage {
        let WindowMessageKind::Splash(message) = self else { panic!("Somehow got invalid workspace event") };
        message
    }
}

impl Into<Message> for SplashWindowMessage {
    fn into(self) -> Message {
        WindowMessage {
            kind: WindowMessageKind::Splash(self.clone()),
            source_id: None,
        }.into()
    }
}


#[derive(Debug)]
pub struct SplashWindow {
    pub id: Id,
    frames: Frames,
    message: String,
    tick: bool,
    progress: f32,
}


const SPLASH_MESSAGES: [&'static str;12] = [
    "Loading",
    "Plucking feathers",
    "Nibbling breadcrumbs",
    "QUACK",
    "Getting our ducks in a row",
    "Quacking the code",
    "Winging it",
    "Waddling over",
    "Parsing Duck-uments",
    "Swimming in data streams",
    "Duck-crypting data",
    "Quack-tivating databases",
];

impl SplashWindow {
    pub fn new() -> (Self, IcedTask<Message>) {
        let (id, task) = window::open(Self::settings());
        let frames = Frames::from_bytes(SPLASH_ART.into());

        if frames.is_err() {
            error!("SplashWindow::new: Failed to decode gif");
            error!("Passthrough: {:?}", frames.err().unwrap());
            panic!("SplashWindow::new: Failed to decode gif");
        }

        (
            Self { 
                id,
                frames: frames.unwrap(),
                tick: true,
                message: SPLASH_MESSAGES[0].to_string(),
                progress: 0.
            },
            task.discard()
        )
    }

    pub fn subscribe(&self) -> Subscription<Message> {
        if !self.tick {
            Subscription::none()
        } else {
            // This adds a fake 3 second loading splash screen
            // It does nothing, but will make the user feel 
            // like we're doing important things, thus trusting
            // us more.
            time::every(Duration::from_millis(200)).map(|_| {
                SplashWindowMessage::splash_survived().into()
            })
        }
    }
}


impl DesktopWindow<SplashWindow, SplashWindowMessage, Message> for SplashWindow {


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
        Theme::SolarizedDark
    }

    fn update(&mut self, event: SplashWindowMessage) -> Task {
        match event.kind {
            SplashWindowMessageKind::StatusMessage(_status) => Task::none(),
            SplashWindowMessageKind::SplashSurvived => {
                if self.progress >= 100. {
                    self.tick = false;
                    return self.close().chain(Message::tick().into())
                }

                self.progress += 4.;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element {
        stack!(
            container(gif(&self.frames))
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            container(
                stack!(
                    progress_bar(0.0..=100.0, self.progress),
                    row!(center(text(self.message.clone())))
                )
            )
        ).into()
    }
    
    fn close(&mut self) -> Task {
        window::close(self.id).chain(Task::done(Message::window_close(self.id)))
    }
}

use iced::{Theme, Task as IcedTask};
use iced::widget::{container, text};
use iced::window::{Id, Settings};
use crate::runtime::{Element, Task};
use crate::runtime::messaging::{Message, WindowMessage};
use crate::runtime::windows::editor::EditorWindow;
use crate::runtime::windows::splash::SplashWindow;
use crate::runtime::windows::workspace::WorkspaceWindow;

pub mod splash;
pub mod workspace;
pub mod editor;

pub mod settings;

/// An enum representing the different windows that the application may display
#[derive(Debug)]
pub enum AppWindow {
    /// An Ignored enum variant which is only used for implementing traits
    Ticker(Id),

    /// A debugging window which shows a view of the current application state
    #[cfg(debug_assertions)]
    DebugWindow,

    /// A window which renders a looping gif while the app starts
    SplashWindow(SplashWindow),

    /// A window for managing workspaces
    WorkspaceWindow(WorkspaceWindow),

    /// A window which contains a workspace allowing editing
    EditorWindow(EditorWindow),

    /// A window which manages the user settings
    SettingsWindow
}

impl DesktopWindow<AppWindow, WindowMessage, Message> for AppWindow {


    fn settings() -> Settings {
        Default::default()
    }

    fn title(&self) -> String {
        match self {
            AppWindow::DebugWindow => "Debug Window".to_string(),
            AppWindow::SplashWindow(window) => window.title(),
            AppWindow::WorkspaceWindow(window) => window.title(),
            AppWindow::EditorWindow(window) => window.title(),
            AppWindow::SettingsWindow => "Settings Window".to_string(),
            _ => "ILLEGITIMATE CHILD WINDOW".to_string()
        }
    }

    fn theme(&self) -> Theme {
        match self {
            AppWindow::SplashWindow(window) => window.theme(),
            _ => Theme::SolarizedDark,
        }
    }

    fn update(&mut self, message: WindowMessage) -> Task {
        // Unwrap our message because we know it is actually a WindowMessage as
        // it reached this point in the code

        match self {
            AppWindow::DebugWindow => Task::none(),
            AppWindow::SplashWindow(window) => window.update(message.kind.into()),
            AppWindow::WorkspaceWindow(window) => window.update(message.kind.into()),
            AppWindow::EditorWindow(window) => window.update(message.kind.into()),
            AppWindow::SettingsWindow => Task::none(),
            _ => Task::none()
        }
    }

    fn view(&self) -> Element {
        match self {
            AppWindow::SplashWindow(window) => window.view(),
            AppWindow::WorkspaceWindow(window) => window.view(),
            AppWindow::EditorWindow(window) => window.view(),
            kind => container(text(format!("Not Implemented {:?}", kind))).into()
        }
    }

    fn close(&mut self) -> Task {
        match self {
            AppWindow::SplashWindow(window) => window.close(),
            AppWindow::EditorWindow(window) => window.close(),
            _ => Task::none(),
        }
    }
}

pub trait DesktopWindow<T, MessageInput, MessageOutput> {
    // fn new() -> (T, IcedTask<Id>);

    fn settings() -> Settings;

    /// A method returning the title of the specific window, based on its state
    fn title(&self) -> String;

    /// A method returning the theme of the specific window
    fn theme(&self) -> Theme;

    /// The method for passing messages to the window,
    /// allowing it to react to events that have happened
    fn update(&mut self, message: MessageInput) -> IcedTask<MessageOutput>;

    /// A method for rendering the window based on its internal state
    fn view(&self) -> Element;

    /// A method to provide a graceful shutdown to the window,
    /// allowing it to preserve any data before closing
    /// > NOTE: This method __MUST__ close the window itself, otherwise the window will remain open.
    /// > It must also return a `Message::WindowClosed(id)` event, so that the app may update
    /// > its own internal state
    fn close(&mut self) -> IcedTask<MessageOutput>;
}

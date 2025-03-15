use std::collections::BTreeMap;
use crate::app::GlobalEvent;
use crate::components::table::row::TableRow;
use crate::components::table::Table;
use crate::consts::{APP_BUILD, APP_ICON, APP_NAME, APP_VERSION};
use iced::widget::container;
use iced::window::{icon, Id, Settings};
use iced::{window, Element, Size, Task};

#[derive(Debug)]
pub struct BuildInfoWindow {
    id: Id,
    stats: Table,
    state: BTreeMap<String, String>,
}

#[derive(Debug)]
pub enum BuildInfoMessage {
    // UpdateTheme(),
    RequestExit,
    // Used for setting the value of a state name in the debug screen
    SetState(String, String),
}



impl BuildInfoWindow {
    pub fn new() -> (Self, Id, Task<Id>) {
        let (id, task) = window::open(Self::config());
        let mut state = Self {
            id,
            stats: Table::new(),
            state: BTreeMap::new(),
        };
        state.stats = state.stats.headers(TableRow::new(vec!["Field", "Value"]));
        state.state.insert("Name".to_string(), APP_NAME.to_string());
        state.state.insert("Version".to_string(), APP_VERSION.to_string());
        state.state.insert("Build ID".to_string(), APP_BUILD.to_string());

        state.redraw();



        (state, id, task)
    }

    pub fn config() -> Settings {
        Settings {
            size: Size { width: 400., height: 400. },
            position: Default::default(),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: false,
            decorations: true,
            transparent: false,
            level: Default::default(),
            icon: Some(icon::from_file_data(APP_ICON, None).unwrap()),
            platform_specific: Default::default(),
            exit_on_close_request: true,
        }
    }

    pub fn update(&mut self, message: BuildInfoMessage) -> Task<GlobalEvent> {
        match message {
            // Handle shutting the window down gracefully
            BuildInfoMessage::RequestExit => Task::done(GlobalEvent::WindowClosed(self.id)),
            BuildInfoMessage::SetState(k,v) => {
                self.state.insert(k, v);
                self.redraw();
                Task::none()
            }
        }
    }

    pub fn redraw(&mut self) {
        self.stats.rows.clear();

        for (key, val) in &self.state {
            self.stats.rows.push(
                TableRow::new(vec![key.clone(), val.clone()])
            )
        }
    }

    pub fn view(&self, id: Id) -> Element<GlobalEvent> {
        container(self.stats.view())
            .padding(10.)
            .into()
    }
}
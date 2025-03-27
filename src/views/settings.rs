use std::sync::Arc;
use iced::{Border, Color, Element};
use iced::border::Radius;
use iced::widget::{column, container, horizontal_rule, horizontal_space, row, scrollable, text, text_input};
use crate::app::GlobalEvent;
use crate::components::form::text_input::TextInput;
use crate::consts::{HEADER_SIZE_2, HEADER_SIZE_3};
use crate::filesystem::config::Config;
use crate::filesystem::workspace::global::WorkspaceManifest;

#[derive(Debug)]
pub struct Settings {
    cfg: Arc<Config>,
    wsp: Arc<WorkspaceManifest>,
    pub show: bool,
}


#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ContentChanged(String, String)
}


impl Settings {
    pub fn toggle(&mut self) {
        self.show = !self.show;
    }

    pub fn new(cfg: Arc<Config>, wsp: Arc<WorkspaceManifest>) -> Self {
        Self { cfg, wsp, show: false}
    }

    pub fn update(&mut self, msg: SettingsMessage) {
        match msg {
            SettingsMessage::ContentChanged(field, value) => {
                match field {
                    _ => {
                        error!("Unrecognized settings field")
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<GlobalEvent> {
        container(
            scrollable(
                column!(
                    header("Settings"),
                    section_header("Workspace Settings")

                )
            )
        ).padding(10.).style(|_theme| {
            container::Style {
                background: Some(
                    Color::BLACK.into()
                ),
                border: Border {
                    color: Color::from_rgb8(0xA1, 0xA1, 0xA1),
                    width: 1.,
                    radius: Radius::new(5),
                },
                ..container::Style::default()
            }
        }).width(500.).height(700.).into()
    }
}


fn section_header<'a>(name: impl Into<String>) -> Element<'a, GlobalEvent> {
    column!(
        text(name.into()).size(HEADER_SIZE_3),
        horizontal_rule(2),
    ).into()
}

fn header<'a>(title: impl Into<String>) -> Element<'a, GlobalEvent> {
    text(title.into()).size(HEADER_SIZE_2).into()
}

fn text_field(key: impl Into<String>, bind: &str) -> Element<GlobalEvent> {
    row!(text(key.into()), text_input(bind, bind)).into()
}
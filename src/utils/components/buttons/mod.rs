use std::sync::Arc;
use iced::{color, Background, Border, Color, Element, Length, Shadow, Theme};
use iced::border::Radius;
use iced::widget::{button, center, horizontal_space, row, text, Text};
use iced::widget::button::Style;
use iced::window::Id;
use crate::consts::{BUTTON_DEFAULT_BACKGROUND, BUTTON_DEFAULT_TEXT, FONT_ICON};
use crate::runtime::messaging::Message;
use crate::runtime::Task;
use crate::utils::components::{ComponentMessage, ComponentRegistration, COMPONENT_TRACKER};
use crate::utils::components::icons::Icon;

#[derive(Clone, Debug)]
pub enum ButtonMessageKind {
    MouseOver(String),
    MouseOut(String),
    Clicked(String),
}

#[derive(Clone, Debug)]
pub struct ButtonMessage {
    pub kind: ButtonMessageKind,
    pub window_id: Id,
}

impl Into<Message> for ButtonMessage {
    fn into(self) -> Message {
        ComponentMessage::button(self).into()
    }
}

impl ButtonMessage {
    pub fn new(kind: ButtonMessageKind, window_id: Id) -> Self {
        ButtonMessage { kind, window_id }
    }

    fn from_reg(reg: Arc<ComponentRegistration>) -> Self {
        Self::new(ButtonMessageKind::Clicked(reg.component_id.clone()), reg.window_id)
    }

    pub fn click(reg: Arc<ComponentRegistration>) -> Self {
        Self::from_reg(reg)
    }

    pub fn mouse_out(reg: Arc<ComponentRegistration>) -> Self {
        Self::from_reg(reg)
    }

    pub fn mouse_over(reg: Arc<ComponentRegistration>) -> Self {
        Self::from_reg(reg)
    }
}

pub struct Button<T: Into<Message>> {
    registration: Arc<ComponentRegistration>,
    on_click: Option<fn() -> T>,
    disabled: bool,
    hovered: bool,
    text: String,
    icon: Option<Icon>
}

impl Button<Message> {
    pub fn new<T: Into<String>>(text: T, id: Id) -> Button<Message> {
        let reg = COMPONENT_TRACKER.lock().unwrap().register(id);
        Self {
            registration: reg,
            on_click: None,
            disabled: false,
            hovered: false,
            text: text.into(),
            icon: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, on_click: fn() -> Message) -> Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn update(&mut self, message: ButtonMessage) -> Task {
        match message.kind {
            ButtonMessageKind::MouseOver(text) => {
                if self.registration.component_id == text {
                    self.hovered = true;
                }
                Task::none()
            },
            ButtonMessageKind::MouseOut(text) => {
                if self.registration.component_id == text {
                    self.hovered = false;
                }
                Task::none()
            },
            ButtonMessageKind::Clicked(text) => {
                if self.registration.component_id == text {
                    return match self.on_click {
                        Some(on_click) => Task::done(on_click()),
                        _ => Task::none()
                    }
                }
                Task::none()
            }
        }


    }

    pub fn view<T>(&self) -> Element<'_, Message> {
        match &self.icon {
            Some(icon) => {

                button(row!(icon.view(), text(self.text.clone())))
                    .into()
            },
            None => {

                button(text(self.text.clone()))
                    .into()
            }
        }

    }
}


pub struct ButtonStyle {
    background: Option<Background>,
    color: Option<Color>,
    border: Option<Border>,
    shadow: Option<Shadow>,
}

impl ButtonStyle {
    pub fn new() -> Self {
        ButtonStyle {
            background: Some(Background::Color(color!(BUTTON_DEFAULT_BACKGROUND))),
            color: None,
            border: None,
            shadow: None,
        }
    }
    pub fn with_background_color(&mut self, color: i32) -> &mut Self {
        self.background = Some(Background::Color(color!(color)));
        self
    }

    pub fn with_background(&mut self, bg: Background) -> &mut Self {
        self.background = Some(bg);
        self
    }

    pub fn with_color(&mut self, color: i32) -> &mut Self {
        self.color = Some(color!(color));
        self
    }

    pub fn with_border(&mut self, border: Border) -> &mut Self {
        self.border = Some(border);
        self
    }

    pub fn with_shadow(&mut self, shadow: Shadow) -> &mut Self {
        self.shadow = Some(shadow);
        self
    }

    pub fn compile(&self) -> Style {
        button_style(self.background, self.color, self.border, self.shadow)
    }
}

pub fn button_style(background: Option<Background>, color: Option<Color>, border: Option<Border>, shadow: Option<Shadow>) -> Style {
    let background = Some(background.unwrap_or(Background::Color(color!(BUTTON_DEFAULT_BACKGROUND))));
    let text_color = color.unwrap_or(color!(BUTTON_DEFAULT_TEXT));
    let border = border.unwrap_or(Border {
        color: color!(0x1a1a1a),
        width: 1.,
        radius: Radius::new(5.),
    });

    Style {
        background,
        text_color,
        border,
        shadow: Default::default(),
    }
}


pub fn button_with_icon<'a, T: Into<String>>(icon: material_icons::Icon, message: T) -> iced::widget::Button<'a, Message> {
    button(
        center(
            row!(
                text(material_icons::icon_to_char(icon)).font(FONT_ICON).size(18),
                horizontal_space().width(5.),
                text(message.into()).size(16)
            )
        ).height(Length::Shrink).width(Length::Fill)
    ).style(|_,_| ButtonStyle::new().compile())
}
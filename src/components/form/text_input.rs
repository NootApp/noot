use crate::events::types::Message;
use iced::Element;
use iced::widget::container;
use iced::widget::{column, text};
use nanoid::nanoid;

#[derive(Debug, Clone)]
pub struct TextInput {
    pub field_id: String,
    pub label: String,
    pub placeholder: Option<String>,
    pub value: String,
    pub obfuscated: bool,
}

#[derive(Debug, Clone)]
pub enum TextInputChanged {}

impl TextInput {
    pub fn new<S: Into<String>, V: Into<String>>(
        label: S,
        value: V,
        obfuscated: bool,
    ) -> TextInput {
        TextInput {
            field_id: nanoid!(),
            label: label.into(),
            placeholder: None,
            value: value.into(),
            obfuscated,
        }
    }

    pub fn placeholder<S: Into<String>>(mut self, placeholder: S) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.value = value.into();
        self
    }

    pub fn obfuscated(mut self, obfuscated: bool) -> Self {
        self.obfuscated = obfuscated;
        self
    }

    pub fn view(&self) -> Element<Message> {
        container(column![text(self.label.clone()), self.text_input()]).into()
    }

    pub fn text_input(&self) -> Element<Message> {
        match &self.placeholder {
            Some(v) => {
                iced::widget::text_input::TextInput::new(v, &self.value).into()
            }
            None => iced::widget::text_input::TextInput::new("", &self.value)
                .on_input(move |c: String| self.on_input(c))
                .into(),
        }
    }

    pub fn on_input(&self, content: String) -> Message {
        Message::FormContentChanged(self.field_id.clone(), content)
    }
}

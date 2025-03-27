use iced::{Element, Length};
use iced::widget::{container, text};
use crate::app::GlobalEvent;
use crate::consts::{FONT_BOLD, FONT_BOLD_ITALIC, FONT_ITALIC, FONT_MEDIUM};
use crate::markdown::TextModifier;

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Text,
    Heading(usize),
    SoftBreak,
    HardBreak,
    Paragraph,
}

#[derive(Debug, Clone)]
pub struct MarkdownToken {
    pub kind: Kind,
    pub id: Option<String>,
    pub content: Option<String>,
    pub modifier: TextModifier,
    pub children: Vec<MarkdownToken>,
}

impl MarkdownToken {
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            id: None,
            content: None,
            modifier: TextModifier::NONE,
            children: Vec::new(),
        }
    }

    pub fn view(&self) -> Element<GlobalEvent> {

        match self.kind {
            Kind::Heading(level) => {
                if self.content.is_none() {
                    return text("".to_string()).into()
                }

                let mut heading = text(self.content.clone().unwrap()).font(FONT_BOLD);

                match level {
                    1 => heading = heading.size(72.),
                    2 => heading = heading.size(48.),
                    3 => heading = heading.size(36.),
                    4 => heading = heading.size(32.),
                    _ => heading = heading.size(24.),
                }

                heading.into()
            }
            Kind::Paragraph => {
                if self.content.is_none() {
                    return text("".to_string()).into()
                }
                let mut c = text(self.content.clone().unwrap());


                if self.modifier.contains(TextModifier::BOLD) && self.modifier.contains(TextModifier::ITALIC) {
                    c = c.font(FONT_BOLD_ITALIC);
                } else if self.modifier.contains(TextModifier::BOLD) {
                    c = c.font(FONT_BOLD);
                } else if self.modifier.contains(TextModifier::ITALIC) {
                    c = c.font(FONT_ITALIC);
                } else {
                    c = c.font(FONT_MEDIUM);
                }

                c.into()
            },

            Kind::SoftBreak => {
                container(text("".to_string())).height(5.).width(Length::Fill).into()
            }

            x => {
                text(format!("{:?} - not implemented", x)).into()
            }
        }

    }
}
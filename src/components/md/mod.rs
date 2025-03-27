use crate::app::GlobalEvent;
use crate::consts::{FONT_BOLD, FONT_BOLD_ITALIC, FONT_ITALIC, FONT_MEDIUM, HEADER_SIZE_1, HEADER_SIZE_2, HEADER_SIZE_3, HEADER_SIZE_4, HEADER_SIZE_5, TEXT_SIZE};
use crate::markdown::TextModifier;
use iced::widget::{container, horizontal_rule, row, text};
use iced::{Color, Element, Font, Length};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Text,
    Heading(usize),
    SoftBreak,
    HardBreak,
    Paragraph,
    Rule,
}

#[derive(Debug, Clone)]
pub struct MarkdownToken {
    pub kind: Kind,
    pub id: Option<String>,
    pub content: Vec<TextToken>,
    pub modifier: TextModifier,
    pub children: Vec<MarkdownToken>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TextToken {
    pub modifier: TextModifier,
    pub content: String,
}

impl MarkdownToken {
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            id: None,
            content: vec![],
            modifier: TextModifier::NONE,
            children: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn view(&self) -> Element<GlobalEvent> {
        match self.kind {
            Kind::Heading(level) => {
                if self.content.len() == 0 {
                    return text("".to_string()).into()
                }

                match level {
                    1 => self.render_text(Some(HEADER_SIZE_1), Some(FONT_BOLD), None),
                    2 => self.render_text(Some(HEADER_SIZE_2), Some(FONT_BOLD), None),
                    3 => self.render_text(Some(HEADER_SIZE_3), Some(FONT_BOLD), None),
                    4 => self.render_text(Some(HEADER_SIZE_4), Some(FONT_BOLD), None),
                    5 => self.render_text(Some(HEADER_SIZE_5), Some(FONT_BOLD), None),
                    _ => self.render_text(Some(TEXT_SIZE), Some(FONT_BOLD), None),
                }
            }
            Kind::Paragraph => {
                if self.content.len() == 0 {
                    return text("".to_string()).into()
                }
                self.render_text(None, Some(FONT_MEDIUM), None)
            },

            Kind::SoftBreak => {
                container(text("".to_string())).height(5.).width(Length::Fill).into()
            }

            Kind::Rule => {
                horizontal_rule(3.).into()
            }

            x => {
                text(format!("{:?} - not implemented", x)).into()
            }
        }
    }

    fn render_text(&self, size: Option<f32>, font: Option<Font>, color: Option<Color>) -> Element<GlobalEvent> {
        if self.content.len() == 0 {
            return text("".to_string()).into()
        }

        let mut tokens: Vec<Element<GlobalEvent>> = vec![];

        for node in &self.content {
            let mut c = text(node.content.clone());

            if let Some(font) = font {
                c = c.font(font);
            }

            if node.modifier.contains(TextModifier::BOLD) && node.modifier.contains(TextModifier::ITALIC) {
                c = c.font(FONT_BOLD_ITALIC);
            } else if node.modifier.contains(TextModifier::BOLD) {
                c = c.font(FONT_BOLD);
            } else if node.modifier.contains(TextModifier::ITALIC) {
                c = c.font(FONT_ITALIC);
            } else {
                c = c.font(FONT_MEDIUM);
            }

            if let Some(unit) = size {
                c = c.size(unit);
            }


            if let Some(tone) = color {
                c = c.color(tone);
            }

            tokens.push(c.into())
        }

        row(tokens).width(Length::Fill).into()
    }
}
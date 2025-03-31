use crate::app::{Element, GlobalEvent};
use crate::consts::{FONT_BOLD, FONT_BOLD_ITALIC, FONT_ITALIC, FONT_MEDIUM, HEADER_SIZE_1, HEADER_SIZE_2, HEADER_SIZE_3, HEADER_SIZE_4, HEADER_SIZE_5, TEXT_SIZE};
use iced::widget::{column, container, horizontal_rule, mouse_area, rich_text, row, span, text, Text};
use iced::{Color, Font, Length};
use std::collections::BTreeMap;
use bitflags::bitflags;
use iced::widget::text::{Rich, Span};


bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
    pub struct TextModifier: u8 {
        const NONE          = 0b0000_0000;
        const BOLD          = 0b0000_0001; // double asterisk
        const ITALIC        = 0b0000_0010; // single asterisk
        const UNDERLINE     = 0b0000_0100;
        const MONOSPACED    = 0b0000_1000;

        // Non standard
        const STRIKETHROUGH = 0b0001_0000;
        const SUPERSCRIPT   = 0b0010_0000;
        const SUBSCRIPT     = 0b0100_0000;
    }
}

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


impl TextToken {
    pub fn render(&self, size: Option<f32>, font: Option<Font>, color: Option<Color>) -> Element {
        let mut c = text(self.content.clone());

        if let Some(font) = font {
            c = c.font(font);
        }

        if self.modifier.contains(TextModifier::BOLD) && self.modifier.contains(TextModifier::ITALIC) {
            c = c.font(FONT_BOLD_ITALIC);
        } else if self.modifier.contains(TextModifier::BOLD) {
            c = c.font(FONT_BOLD);
        } else if self.modifier.contains(TextModifier::ITALIC) {
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

        c.into()
    }
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

    pub fn view(&self) -> Element {
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

    fn render_text(&self, size: Option<f32>, font: Option<Font>, color: Option<Color>) -> Element {
        // let mut chars = 0;
        // let mut rows = vec![];

        if self.content.len() == 0 {
            return text("".to_string()).into()
        }

        // let mut tokens: Vec<Span> = vec![];
        let mut tokens: Vec<Element> = vec![];
        for node in &self.content {
            // if node.content.len() > 80 {
            //     let mut node = node.clone();
            //     let mut parts: Vec<String> = vec![];
            //     while node.content.len() > 80 {
            //         parts.push(node.content.drain(0..80).collect());
            //     }
            //
            //     for part in parts {
            //         let new_node = TextToken {
            //             modifier: node.modifier,
            //             content: part,
            //         };
            //
            //         rows.push(row!(new_node.render(size, font, color)).into());
            //         chars = 0;
            //     }
            //
            //     continue;
            // } else if chars + node.content.len() > 80 {
            //     rows.push(row(tokens).width(Length::Fill).into());
            //     tokens = vec![];
            //     chars = 0;
            // }

            tokens.push(node.render(size, font, color));
            // chars += node.content.len();
        }


        // rows.push(row(tokens).width(Length::Fill).into());

        row(tokens).width(Length::Fill).into()
        // mouse_area(rich_text(tokens).width(Length::Fill)).into()
    }
}
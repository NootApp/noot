use iced::window::{icon, Id};
use iced::{Element, Length, Size, Task, Theme};
use chrono::{DateTime, Local};
use std::path::PathBuf;
use iced::widget::{button, text, column, horizontal_space, row, scrollable, text_editor, Container};
use pulldown_cmark::{Event, Tag, TagEnd};
use crate::app::GlobalEvent;
use crate::components::md::{Kind, MarkdownToken, TextToken};
use crate::markdown::TextModifier;
use crate::windows::editor_window::EditorEvent;

#[derive(Debug)]
pub struct EditorPanel {
    pub window_id: Id,
    pub preview: Vec<MarkdownToken>,
    pub editor: text_editor::Content,
    pub file: PathBuf,
    pub tab_title: String,
    pub mode: PanelMode
}

#[derive(Debug)]
pub enum PanelMode {
    EditorOnly,
    Combined,
    PreviewOnly
}

impl EditorPanel {
    pub fn new_from_bytes(id: Id, path: PathBuf, bytes: &[u8]) -> Self {
        let text = String::from_utf8(bytes.to_vec()).unwrap();
        Self::new(id, path, text)
    }

    pub fn new(id: Id, path: PathBuf, text: String) -> Self {
        let mut panel = Self {
            window_id: id,
            preview: vec![],
            editor: text_editor::Content::new(),
            file: path.clone(),
            tab_title: path.file_name().unwrap().to_str().unwrap().to_string(),
            mode: PanelMode::Combined,
        };

        error!("Editor path: {}", path.display());

        if path.starts_with("noot://") {
            panel.mode = PanelMode::PreviewOnly;
            panel.tab_title = path.to_str().unwrap().to_string();
        }

        let mut active_modifiers = TextModifier::NONE;

        let mut nodes: Vec<MarkdownToken> = vec![
            MarkdownToken::new(Kind::Paragraph)
        ];
        let parser = pulldown_cmark::Parser::new(&text);
        let mut current_content_string = "".to_string();
        for event in parser {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Heading { level, id: _, classes: _, attrs: _ } => nodes.push(MarkdownToken::new(Kind::Heading(level as usize))),
                        Tag::Paragraph => nodes.push(MarkdownToken::new(Kind::Paragraph)),
                        Tag::Emphasis => active_modifiers = active_modifiers | TextModifier::ITALIC,
                        Tag::Strong => active_modifiers = active_modifiers | TextModifier::BOLD,
                        Tag::Strikethrough => active_modifiers = active_modifiers | TextModifier::STRIKETHROUGH,
                        Tag::Subscript => active_modifiers = active_modifiers | TextModifier::SUBSCRIPT,
                        Tag::Superscript => active_modifiers = active_modifiers | TextModifier::SUPERSCRIPT,
                        _ => {
                            warn!("Tag {:?} is unknown", tag);
                        }
                    }
                }
                Event::End(tag) => {
                    let mut current = nodes.last_mut().unwrap();
                    match tag {
                        TagEnd::Heading(_) => {
                            current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
                            current_content_string = "".to_string();
                        },
                        TagEnd::Paragraph => {
                            current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
                            current_content_string = "".to_string();
                        }
                        TagEnd::Emphasis => {
                            active_modifiers.remove(TextModifier::ITALIC);
                            current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
                            current_content_string = "".to_string();
                        },
                        TagEnd::Strong => {
                            active_modifiers.remove(TextModifier::BOLD);
                            current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
                            current_content_string = "".to_string();
                        },
                        TagEnd::Strikethrough => {
                            active_modifiers.remove(TextModifier::STRIKETHROUGH);
                            current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
                            current_content_string = "".to_string();
                        },
                        TagEnd::Subscript => {
                            active_modifiers.remove(TextModifier::SUBSCRIPT);
                            current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
                            current_content_string = "".to_string();
                        },
                        TagEnd::Superscript => {
                            active_modifiers.remove(TextModifier::SUPERSCRIPT);
                            current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
                            current_content_string = "".to_string();
                        },
                        _=> {
                            warn!("Tag Ending {:?} is unknown", tag);
                        }
                    }

                }
                Event::Text(content) => {
                    current_content_string = format!("{}{}", current_content_string, content);
                },
                Event::SoftBreak => {
                    if current_content_string.len() > 1 {
                        let current = nodes.last_mut().unwrap();

                        current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
                        active_modifiers = TextModifier::NONE;
                        current_content_string = "".to_string();
                    }
                },
                Event::Rule => {
                    let current = nodes.last_mut().unwrap();
                    current.kind = Kind::Rule
                }
                _ => {
                    warn!("Unsupported token type: {:?}", event);
                }
            }
        }
        panel.editor = text_editor::Content::with_text(&text);
        panel.preview = nodes;

        panel
    }

    pub fn view(&self) -> Element<GlobalEvent> {
        match self.mode {
            PanelMode::PreviewOnly => {
                row!(
                    scrollable(
                        column(
                            self.preview.iter().map(|token| token.view())
                        ).width(Length::Fill)
                    ).height(Length::Fill),
                ).height(Length::Fill).into()
            },
            PanelMode::Combined => {
                row!(
                    column!(text_editor(&self.editor).on_action(|e| {
                        GlobalEvent::Editor(self.window_id, EditorEvent::Edit(self.file.clone(), e))
                    }).height(Length::Fill)).width(Length::Fill).height(Length::Fill),
                    scrollable(
                        column(
                            self.preview.iter().map(|token| token.view())
                        ).width(Length::Fill)
                    ).height(Length::Fill),
                ).height(Length::Fill).into()
            },
            PanelMode::EditorOnly => {
                row!(
                    column!(text_editor(&self.editor).on_action(|e| {
                        GlobalEvent::Editor(self.window_id, EditorEvent::Edit(self.file.clone(), e))
                    }).height(Length::Fill)).width(Length::Fill).height(Length::Fill),
                ).height(Length::Fill).into()
            }
        }

    }

    pub fn view_tab(&self) -> Element<GlobalEvent> {
        row!(
            button(
                row!(
                    horizontal_space(),
                    text(self.tab_title.as_str()),
                    horizontal_space(),
                )
            ).on_press_with(|| {
                GlobalEvent::Editor(self.window_id, EditorEvent::FocusFile(self.file.clone()))
            }),
            button(text("x").width(10.)).on_press_with(|| {
                GlobalEvent::Editor(self.window_id, EditorEvent::CloseFile(self.file.clone()))
            })
        ).width(150.).into()
    }

    pub fn close(&self) {} // currently does nothing
}


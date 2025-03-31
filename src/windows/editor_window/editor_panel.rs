use std::fmt::Debug;
use std::fs;
use iced::window::{icon, Id};
use iced::{Length, Size, Subscription, Task, Theme};
use chrono::{DateTime, Local};
use std::path::PathBuf;
use html_parser::{Dom, DomVariant, Node};
use iced::widget::{button, text, column, horizontal_space, row, scrollable, text_editor, Container, Column};
use iced_webview::{Ultralight, WebView};
use pulldown_cmark::{Event, Tag, TagEnd};
use crate::app::{GlobalEvent, Element};
use crate::components::md::{Kind, MarkdownToken, TextToken, TextModifier};
use crate::consts::FONT_MONO;
use crate::windows::editor_window::{EditorEvent};

pub struct EditorPanel<'a> {
    pub window_id: Id,
    pub preview: Dom,
    pub editor: text_editor::Content,
    pub file: PathBuf,
    pub tab_title: String,
    pub mode: PanelMode,
    pub rendered: Column<'a, GlobalEvent>
    // pub webview: WebView<Ultralight, GlobalEvent>,
}

impl Debug for EditorPanel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EditorPanel")
        .field("window_id", &self.window_id)
        .field("preview", &self.preview)
        .field("editor", &self.editor)
        .field("file", &self.file)
        .field("tab_title", &self.tab_title)
        .field("mode", &self.mode)
        .field("webview", &"Webview is not debuggable".to_string())
            .finish()
    }
}


#[derive(Debug)]
pub enum PanelMode {
    EditorOnly,
    Combined,
    PreviewOnly
}

impl EditorPanel<'_> {
    pub fn new_from_bytes(id: Id, path: PathBuf, bytes: &[u8]) -> Self {
        let text = String::from_utf8(bytes.to_vec()).unwrap();
        Self::new(id, path, text)
    }

    pub fn new(id: Id, path: PathBuf, text: String) -> Self {
        let mut panel = Self {
            window_id: id,
            preview: Dom::default(),
            editor: text_editor::Content::new(),
            file: path.clone(),
            tab_title: path.file_name().unwrap().to_str().unwrap().to_string(),
            mode: PanelMode::Combined,
            // webview: Default::default(),
            rendered: column!(""),
        };

        error!("Editor path: {}", path.display());

        if path.starts_with("noot://") {
            panel.mode = PanelMode::PreviewOnly;
            panel.tab_title = "INTERNAL PAGE".to_string();
        }

        // let mut active_modifiers = TextModifier::NONE;
        //
        // let mut nodes: Vec<MarkdownToken> = vec![
        //     MarkdownToken::new(Kind::Paragraph)
        // ];


        let options = pulldown_cmark::Options::ENABLE_TABLES
            | pulldown_cmark::Options::ENABLE_FOOTNOTES
            | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
            | pulldown_cmark::Options::ENABLE_TASKLISTS
            | pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION
            | pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES
            | pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
            | pulldown_cmark::Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS
            | pulldown_cmark::Options::ENABLE_MATH
            | pulldown_cmark::Options::ENABLE_GFM
            | pulldown_cmark::Options::ENABLE_DEFINITION_LIST
            | pulldown_cmark::Options::ENABLE_SUPERSCRIPT
            | pulldown_cmark::Options::ENABLE_SUBSCRIPT
            | pulldown_cmark::Options::ENABLE_WIKILINKS;
        
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, pulldown_cmark::Parser::new_ext(&text, options));
        fs::write("./sample.html", html.as_bytes()).unwrap();
        let doc = html_parser::Dom::parse(&html).unwrap();
        dbg!(&doc);
        fs::write("./sample.json", doc.to_json_pretty().unwrap().as_bytes()).unwrap();
        
        // let parser = pulldown_cmark::Parser::new(&text);
        // let mut current_content_string = "".to_string();
        // for event in parser {
        //     match event {
        //         Event::Start(tag) => {
        //             match tag {
        //                 Tag::Heading { level, id: _, classes: _, attrs: _ } => nodes.push(MarkdownToken::new(Kind::Heading(level as usize))),
        //                 Tag::Paragraph => nodes.push(MarkdownToken::new(Kind::Paragraph)),
        //                 Tag::Emphasis => active_modifiers = active_modifiers | TextModifier::ITALIC,
        //                 Tag::Strong => active_modifiers = active_modifiers | TextModifier::BOLD,
        //                 Tag::Strikethrough => active_modifiers = active_modifiers | TextModifier::STRIKETHROUGH,
        //                 Tag::Subscript => active_modifiers = active_modifiers | TextModifier::SUBSCRIPT,
        //                 Tag::Superscript => active_modifiers = active_modifiers | TextModifier::SUPERSCRIPT,
        //                 _ => {
        //                     warn!("Tag {:?} is unknown", tag);
        //                 }
        //             }
        //         }
        //         Event::End(tag) => {
        //             let mut current = nodes.last_mut().unwrap();
        //             match tag {
        //                 TagEnd::Heading(_) => {
        //                     current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
        //                     current_content_string = "".to_string();
        //                 },
        //                 TagEnd::Paragraph => {
        //                     current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
        //                     current_content_string = "".to_string();
        //                 }
        //                 TagEnd::Emphasis => {
        //                     active_modifiers.remove(TextModifier::ITALIC);
        //                     current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
        //                     current_content_string = "".to_string();
        //                 },
        //                 TagEnd::Strong => {
        //                     active_modifiers.remove(TextModifier::BOLD);
        //                     current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
        //                     current_content_string = "".to_string();
        //                 },
        //                 TagEnd::Strikethrough => {
        //                     active_modifiers.remove(TextModifier::STRIKETHROUGH);
        //                     current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
        //                     current_content_string = "".to_string();
        //                 },
        //                 TagEnd::Subscript => {
        //                     active_modifiers.remove(TextModifier::SUBSCRIPT);
        //                     current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
        //                     current_content_string = "".to_string();
        //                 },
        //                 TagEnd::Superscript => {
        //                     active_modifiers.remove(TextModifier::SUPERSCRIPT);
        //                     current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
        //                     current_content_string = "".to_string();
        //                 },
        //                 _=> {
        //                     warn!("Tag Ending {:?} is unknown", tag);
        //                 }
        //             }
        // 
        //         }
        //         Event::Text(content) => {
        //             info!("Character count: {} -> {} | '{}'", current_content_string.len(), content.len(), format!("{}{}", current_content_string, content));
        //             if current_content_string.len() + content.len() > 80 {
        //                 let current = nodes.last_mut().unwrap();
        //                 current.content.push(TextToken { modifier: active_modifiers, content: current_content_string });
        //                 current_content_string = "".to_string();
        //             }
        //             current_content_string = format!("{}{}", current_content_string, content);
        //         },
        //         Event::SoftBreak => {
        //             if current_content_string.len() > 0 {
        //                 let current = nodes.last_mut().unwrap();
        // 
        //                 current.content.push(TextToken { modifier: active_modifiers.clone(), content: current_content_string.clone() });
        //                 active_modifiers = TextModifier::NONE;
        //                 current_content_string = "".to_string();
        //             }
        //         },
        //         Event::Rule => {
        //             let current = nodes.last_mut().unwrap();
        //             current.kind = Kind::Rule
        //         }
        //         _ => {
        //             warn!("Unsupported token type: {:?}", event);
        //         }
        //     }
        // }
        panel.editor = text_editor::Content::with_text(&text);
        panel.preview = doc;
        panel.render();
        panel
    }

    pub fn update(&mut self, message: EditorEvent) -> Task<GlobalEvent> {
        match message {
            // EditorEvent::WebView => {
            //     let task = self.webview.update(iced_webview::Action::Update);
            //
            //     task
            // }
            _ => Task::none()
        }
    }

    pub fn view(&self) -> Element {
        match self.mode {
            // PanelMode::PreviewOnly => {
            _ => {
                row!(
                    scrollable(
                        column!(
                            self.render()
                            // self.preview.iter().map(|token| token.view())
                        ).width(Length::Fill)
                    ).height(Length::Fill),
                ).height(Length::Fill).into()
            },
            // PanelMode::Combined => {
            //     row!(
            //         column!(text_editor(&self.editor).on_action(|e| {
            //             GlobalEvent::Editor(self.window_id, EditorEvent::Edit(self.file.clone(), e))
            //         }).height(Length::Fill)).width(Length::Fill).height(Length::Fill),
            //         scrollable(
            //             column(
            //                 self.preview.iter().map(|token| token.view())
            //             ).width(Length::Fill)
            //         ).height(Length::Fill),
            //     ).height(Length::Fill).into()
            // },
            // PanelMode::EditorOnly => {
            //     row!(
            //         column!(text_editor(&self.editor).on_action(|e| {
            //             GlobalEvent::Editor(self.window_id, EditorEvent::Edit(self.file.clone(), e))
            //         }).height(Length::Fill)).width(Length::Fill).height(Length::Fill),
            //     ).height(Length::Fill).into()
            // }
        }

    }

    pub fn view_tab(&self) -> Element {
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
        ).into()
    }

    pub fn render(&mut self) {
        let mut nodes: Vec<Element> = vec![];

        match self.preview.tree_type {
            DomVariant::Document => {
                nodes.push(text("Document Found").into())
            }
            DomVariant::DocumentFragment => {
                for node in &self.preview.children {
                    let output = render_node(node);
                    if let Some(content) = output {
                        nodes.push(content);
                        break;
                    }
                }
            }
            DomVariant::Empty => {
                nodes.push(text("Nothing to show").into())
            }
        }

        self.rendered = column(nodes)
    }

    pub fn close(&self) {} // currently does nothing
}


pub fn render_node(node: &Node) -> Option<Element> {
    match node {
        Node::Text(content) => {
            Some(text(content).into())
        },
        Node::Element(element) => {
            dbg!(node);
            Some(text(format!("{:?}", node)).font(FONT_MONO).into())
        },
        Node::Comment(_) => None // Ignore Comments
    }
}
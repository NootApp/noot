use std::path::PathBuf;
use std::collections::HashMap;
use url::Url;
use html_parser::{Dom, Node};
use pulldown_cmark::{Parser, Options};
use iced::{color, Length, Padding};
use iced::widget::{span, rich_text};
use iced::widget::{row, column, tooltip, container, text, horizontal_space, horizontal_rule, mouse_area};

use crate::consts::*;
use crate::runtime::Element;
use crate::runtime::Message;
use crate::runtime::messaging::MessageKind;

#[derive(Debug, Clone)]
pub struct Buffer {
    pub name: String,
    pub url: Url,
    pub doc: Vec<ElWrapper>,
}

impl From<PathBuf> for Buffer {
    fn from(p: PathBuf) -> Buffer {
        Buffer::new("".into(), p.to_str().unwrap(), "".into())
    }
}

impl Buffer {
    pub fn new<U: Into<String>>(name: String, url: U, content: String) -> Self {
        let dom = Dom::parse(&content).unwrap();
        let mut els = vec![];

        for element in dom.children {
            match element {
                Node::Comment(_) => continue,
                _ => els.push(ElWrapper::new(element))
            }
        }


        Self {
            name,
            url: Url::parse(&url.into()).unwrap(),
            doc: els
        }
    }

    pub fn from_md<U: Into<String>>(name: String, url: U, content: String) -> Self {
        let mut options = Options::empty();

        options.insert(Options::ENABLE_STRIKETHROUGH);

        let parser = Parser::new_ext(&content, options);


        // Write to String buffer.

        let mut html_output = String::new();

        pulldown_cmark::html::push_html(&mut html_output, parser);

        Self::new(name, url, html_output)
    }

    pub fn view(&self) -> Element {
        let start = std::time::Instant::now();
        let view = container(
            column(
                self.doc.iter().map(|e| {
                    e.view()
                })
            )
        )
        .padding(Padding {top:5., right: 20., bottom: 5., left: 5.})
        .width(Length::Fill)
        .into();

        let diff = start.elapsed().as_micros();
        info!("Rendered document in {}μs", diff);

        view
    }
}

#[derive(Debug, Clone)]
pub struct ElWrapper {
    id: Option<String>,
    name: String,
    attributes: HashMap<String, Option<String>>,
    classes: Vec<String>,
    children: Vec<ElWrapper>,
    pub display_text: String
}


impl ElWrapper {
    pub fn new(el: Node) -> Self {
        match el {
            Node::Text(content) => {
                Self {
                    id: None,
                    name: "TEXT".to_string(),
                    attributes: Default::default(),
                    classes: vec![],
                    children: vec![],
                    display_text: content.clone(),
                }
            },
            Node::Element(element) => {
                
                if element.name == "code".to_string() {
                    dbg!(&element);
                }

                let mut children = vec![];

                for child in element.children {
                    children.push(Self::new(child))
                }

                Self {
                    id: element.id.clone(),
                    name: element.name.clone(),
                    attributes: element.attributes.clone(),
                    classes: element.classes.clone(),
                    children,
                    display_text: String::new(),
                }
            }
            _ => {
                Self {
                    id: None,
                    name: "INVALID".to_string(),
                    attributes: Default::default(),
                    classes: vec![],
                    children: vec![],
                    display_text: String::new(),
                }
            }
        }
    }

    pub fn view(&self) -> Element {
        match self.name.as_str() {
            "TEXT" => text(unescape_html_text(self.display_text.as_str())).into(),
            "hr" => horizontal_rule(2).into(),
            "h1" => {
                row(
                    self.children.iter().map(|c| {
                        text(unescape_html_text(c.display_text.as_str()))
                            .font(FONT_BOLD)
                            .size(HEADER_SIZE_1)
                            .into()
                    })
                ).wrap().into()
            },
            "h2" => {
                row(
                    self.children.iter().map(|c| {
                        text(unescape_html_text(c.display_text.as_str()))
                            .font(FONT_BOLD)
                            .size(HEADER_SIZE_2)
                            .into()
                    })
                ).wrap().into()
            },
            "h3" => {
                row(
                    self.children.iter().map(|c| {
                        text(unescape_html_text(c.display_text.as_str()))
                            .font(FONT_BOLD)
                            .size(HEADER_SIZE_3)
                            .into()
                    })
                ).wrap().into()
            },
            "h4" => {
                row(
                    self.children.iter().map(|c| {
                        text(unescape_html_text(c.display_text.as_str()))
                            .font(FONT_BOLD)
                            .size(HEADER_SIZE_4)
                            .into()
                    })
                ).wrap().into()
            },
            "h5" => {
                row(
                    self.children.iter().map(|c| {
                        text(unescape_html_text(c.display_text.as_str()))
                            .font(FONT_BOLD)
                            .size(HEADER_SIZE_5)
                            .into()
                    })
                ).wrap().into()
            },
            "p" => {
                row(self.children.iter().map(|c| c.view())).into()
            }
            "div" => {
                row(
                    self.children.iter().map(|c| {
                        c.view()
                    })
                ).wrap().into()
            },
            "pre" => {
                container(
                    row(
                        self.children[0].children.iter().map(|c| {
                            text(unescape_html_text(c.display_text.as_str()))
                                .font(FONT_MONO)
                                .into()
                        })
                    )
                    .wrap()
                )
                .padding(5)
                .width(Length::Fill)
                .style(|_| {
                    container::Style {
                        background: Some(iced::Background::Color(color!(0x1a1a1a))),
                        border: iced::Border {
                            color: color!(0x2a2a2a),
                            width: 2.,
                            radius: iced::border::Radius::new(5)
                        },
                        .. Default::default()
                    }
                })
                    .into()

            }
            "code" => {
                // All children should be text elements, so we can safely render as text
                container(
                    row(
                        self.children.iter().map(|c| {
                            text(unescape_html_text(c.display_text.as_str()))
                                .font(FONT_MONO)
                                .into()
                        })
                    )
                    .wrap()
                )
                .padding(1)
                .style(|_| {
                    container::Style {
                        background: Some(iced::Background::Color(color!(0x1a1a1a))),
                        border: iced::Border {
                            color: color!(0x2a2a2a),
                            width: 2.,
                            radius: iced::border::Radius::new(5)
                        },
                        .. Default::default()
                    }
                })
                    .into()
            },
            "li" => {
                row(
                    self.children.iter().map(|c| {
                        c.view().into()
                    })
                ).wrap().into()
            },
            "ul" => {
                column(
                    self.children.iter().map(|c| {
                        row!(text("- "), c.view()).wrap().into()
                    })
                ).into()
            }
            "a" => {
                let path_opt = self.attributes.get("href");
                let mut tooltip_text = "No URL Provided".to_string();
                let mut url = None;
                if let Some(maybe_path) = path_opt {
                    if let Some(path) = maybe_path {
                        tooltip_text = path.clone();
                        url = Some(path.clone());
                    }
                }

                mouse_area(
                    tooltip(
                        row(
                            self.children.iter().map(|c| {
                                text(unescape_html_text(c.display_text.as_str())).color(color!(0x0000ff)).into()
                            })
                        ).wrap(), 
                        container(text(tooltip_text)).style(container::rounded_box),
                        tooltip::Position::Bottom
                    )
                )
                    .on_release(Message::new(MessageKind::LinkOpened(url), None))
                    .into()
                
            }
            _ => {
                dbg!(&self);
                text(format!("{} - Not Supported", self.name)).into()
            }
        }
    }
}


fn unescape_html_text(src: &str) -> String {
    let after: String = html_escape::decode_html_entities(src).into();
    after.replace("\t", " ")
}

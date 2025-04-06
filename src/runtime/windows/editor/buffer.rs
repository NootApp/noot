use std::path::PathBuf;
use std::collections::HashMap;
use url::Url;
use html_parser::{Dom, Node};
use pulldown_cmark::{Parser, Options, Event};
use iced::{color, Border, Length, Padding};
use iced::border::Radius;
use iced::widget::{row, column, span, container, text, horizontal_rule};
use iced::widget::text::Span;
use crate::consts::*;
use crate::utils::components::widgets::rich_text;
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

        let mut highlighted: Vec<Event> = vec![];

        for event in parser {
            match event {
                // TODO: Implement code highlighting
                // Event::Start(Tag::CodeBlock(kind)) => {
                //
                // },
                // Event::End(Tag::CodeBlock(_)) => {
                //
                // }
                _ => highlighted.push(event),
            }
        }

        // highlight_with_theme(parser, "base16-ocean.dark").unwrap();

        pulldown_cmark::html::push_html(&mut html_output, highlighted.into_iter());

        Self::new(name, url, html_output)
    }

    pub fn view(&self) -> Element {
        let start = std::time::Instant::now();




        let view = container(
            greedy_render(&self.doc)
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


pub enum Render<'a> {
    Element(Element<'a>),
    Span(Span<'a, Message>)
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

    pub fn view(&self) -> Render {
        match self.name.as_str() {
            "TEXT" =>
                Render::Span(span(unescape_html_text(self.display_text.as_str()))),
            "hr" => Render::Element(horizontal_rule(2).into()),
            "h1" => {
                Render::Element(
                    rich_text(
                        self.children.iter().map(|c| {
                            span(unescape_html_text(c.display_text.as_str()))
                            .font(FONT_BOLD)
                            .size(HEADER_SIZE_1)
                        }).collect::<Vec<Span<Message>>>()
                    ).into()
                )
            },
            "h2" => {
                Render::Element(
                    rich_text(
                        self.children.iter().map(|c| {
                            span(unescape_html_text(c.display_text.as_str()))
                                .font(FONT_BOLD)
                                .size(HEADER_SIZE_2)
                        }).collect::<Vec<Span<Message>>>()
                    ).into()
                )
            },
            "h3" => {
                Render::Element(
                    rich_text(
                        self.children.iter().map(|c| {
                            span(unescape_html_text(c.display_text.as_str()))
                                .font(FONT_BOLD)
                                .size(HEADER_SIZE_3)
                        }).collect::<Vec<Span<Message>>>()
                    ).into()
                )
            },
            "h4" => {
                Render::Element(
                    rich_text(
                        self.children.iter().map(|c| {
                            span(unescape_html_text(c.display_text.as_str()))
                                .font(FONT_BOLD)
                                .size(HEADER_SIZE_4)
                        }).collect::<Vec<Span<Message>>>()
                    ).into()
                )
            },
            "h5" => {
                Render::Element(
                    rich_text(
                        self.children.iter().map(|c| {
                            span(unescape_html_text(c.display_text.as_str()))
                                .font(FONT_BOLD)
                                .size(HEADER_SIZE_5)
                        }).collect::<Vec<Span<Message>>>()
                    ).into()
                )
            },
            "p" => {
                Render::Element(greedy_render(&self.children))
            }
            "div" => {
                Render::Element(greedy_render(&self.children))
            },
            "pre" => {
                Render::Element(
                    container(
                        rich_text(
                            self.children[0].children.iter().map(|c| {
                                span(unescape_html_text(c.display_text.as_str()))
                                    .font(FONT_MONO)
                            }).collect::<Vec<Span<Message>>>()
                        )
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
                )

            }
            "code" => {
                let mut padding = Padding::new(2.);
                padding.left = 3.;
                padding.right = 3.;

                // All children should be text elements, so we can safely render as text
                Render::Span(
                    span(
                        self.children.iter().map(|c| unescape_html_text(c.display_text.as_str()))
                            .collect::<Vec<String>>()
                            .join(" ")
                    ).font(FONT_MONO)
                        .background(color!(0x1a1a1a))
                        .border(Border {
                            color: color!(0x2a2a2a),
                            width: 2.0,
                            radius: Radius::new(5),
                        })
                        .padding(padding)
                )
            },
            "li" => {
                Render::Span(
                    span(format!("- {}", greedy_text(&self.children)))
                )
            },
            "ul" => {
                Render::Element(
                    column(
                        self.children.iter().map(|c| {
                            row!(text(format!("- {}", greedy_text(&c.children)))).wrap().into()
                        })
                    ).into()
                )
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
                Render::Span(
                    span(unescape_html_text(greedy_text(&self.children).as_str()))
                        .link(Message::new(MessageKind::LinkOpened(url), None))
                        .color(color!(0x0000fa))
                )
                // Render::Element(
                //     mouse_area(
                //         tooltip(
                //             rich_text(
                //                 self.children.iter().map(|c| {
                //                     span(unescape_html_text(c.display_text.as_str())).color(color!(0x0000ff))
                //                 }).collect::<Vec<Span<Message>>>()
                //             ),
                //             container(text(tooltip_text)).style(container::rounded_box),
                //             tooltip::Position::Bottom
                //         )
                //     )
                //         .on_release()
                //         .into()
                // )
            }
            _ => {
                dbg!(&self);
                Render::Element(rich_text([span(format!("{} - Not Supported", self.name))]).into())
            }
        }
    }
}


fn unescape_html_text(src: &str) -> String {
    let after: String = html_escape::decode_html_entities(src).into();
    after.replace("\t", " ")
}


fn greedy_render(elements: &[ElWrapper]) -> Element {
    let mut col = vec![];
    let mut spans = vec![];

    for element in elements {
        match element.view() {
            Render::Element(e) => {
                col.push(rich_text(spans).into());
                spans = vec![];
                col.push(e);
            }
            Render::Span(s) => spans.push(s.into()),
        }
    }
    col.push(rich_text(spans).into());


    column(
        col
    ).into()
}

fn greedy_text(elements: &[ElWrapper]) -> String {
    elements.iter().map(|c| unescape_html_text(c.display_text.as_str()))
        .collect::<Vec<String>>()
        .join(" ")
}
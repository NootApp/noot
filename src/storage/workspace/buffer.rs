use std::borrow::Cow;
use std::path::PathBuf;
use std::collections::HashMap;
use url::Url;
use html_parser::{Dom, Node};
use pulldown_cmark::{Parser, Options, Event};
use iced::{color, Border, Length, Padding};
use iced::border::Radius;
use iced::widget::{row, column, span, container, text, horizontal_rule, mouse_area, Svg};
use iced::widget::text::Span;
use iced::advanced::svg::Handle;
use iced_aw::{grid, grid_row};
use iced_core::alignment::Horizontal;
use crate::consts::*;
use crate::utils::components::widgets::rich_text;
use crate::runtime::Element;
use crate::runtime::Message;
use crate::runtime::messaging::MessageKind;
use crate::utils::cryptography::hashing::hash_str;

#[derive(Debug, Clone)]
pub struct Buffer {
    pub id: String,
    pub name: String,
    pub url: Url,
    pub doc: Vec<ElWrapper>,
    // assets: BTreeMap<String, Vec<u8>>
}

pub enum Image {
    Svg(Vec<u8>),
    Other(Vec<u8>)
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
                Node::Text(_) => els.push(ElWrapper::new(element)),
                Node::Element(e) => {
                    match e.name.as_str() {
                        "img" => {
                            dbg!(&e);
                            // let images = Vec<>
                            // for child in e.children {
                            //
                            // }
                            els.push(ElWrapper::new(Node::Element(e)));
                        },
                        _ => els.push(ElWrapper::new(Node::Element(e))),
                    }
                }
            }
        }

        // let asset_base = PathBuf::from(wsm.source.disk_path).with_file_name("assets");

        Self {
            id: nanoid!(5),
            name,
            url: Url::parse(&url.into()).unwrap(),
            doc: els,
            // assets: BTreeMap::new()
        }
    }

    pub fn from_md<U: Into<String>>(name: String, url: U, content: String) -> Self {
        let mut options = Options::empty();

        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SUBSCRIPT);
        options.insert(Options::ENABLE_SUPERSCRIPT);
        options.insert(Options::ENABLE_WIKILINKS);
        options.insert(Options::ENABLE_GFM);
        options.insert(Options::ENABLE_DEFINITION_LIST);
        options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);



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
    // id: Option<String>,
    name: String,
    attributes: HashMap<String, Option<String>>,
    // classes: Vec<String>,
    children: Vec<ElWrapper>,
    pub display_text: String
}


pub enum Render<'a> {
    /// Contains an element, and a boolean stating whether the widget can be inlined or not
    Element(Element<'a>, bool),
    /// Contains a rich text span
    Span(Span<'a, Message>)
}


impl ElWrapper {
    pub fn new(el: Node) -> Self {
        match el {
            Node::Text(content) => {
                Self {
                    // id: None,
                    name: "TEXT".to_string(),
                    attributes: Default::default(),
                    // classes: vec![],
                    children: vec![],
                    display_text: content.clone(),
                }
            },
            Node::Element(element) => {
                let mut children = vec![];

                for child in element.children {
                    children.push(Self::new(child))
                }

                Self {
                    // id: element.id.clone(),
                    name: element.name.clone(),
                    attributes: element.attributes.clone(),
                    // classes: element.classes.clone(),
                    children,
                    display_text: String::new(),
                }
            }
            _ => {
                Self {
                    // id: None,
                    name: "INVALID".to_string(),
                    attributes: Default::default(),
                    // classes: vec![],
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
            "hr" => Render::Element(horizontal_rule(2).into(), false),
            "h1" => heading(greedy_text(&self.children), HEADER_SIZE_1),
            "h2" => heading(greedy_text(&self.children), HEADER_SIZE_2),
            "h3" => heading(greedy_text(&self.children), HEADER_SIZE_3),
            "h4" => heading(greedy_text(&self.children), HEADER_SIZE_4),
            "h5" => heading(greedy_text(&self.children), HEADER_SIZE_5),
            "p" => {
                Render::Element(greedy_render(&self.children), false)
            }
            "div" => {
                Render::Element(greedy_render(&self.children), false)
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
                                radius: Radius::new(5)
                            },
                            .. Default::default()
                        }
                    })
                    .into(),
                    false
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
                    ).into(),
                    false
                )
            }
            "a" => {
                let path_opt = self.attributes.get("href");
                // let mut tooltip_text = "No URL Provided".to_string();
                let mut url = None;
                if let Some(maybe_path) = path_opt {
                    if let Some(path) = maybe_path {
                        // tooltip_text = path.clone();
                        url = Some(path.clone());
                    }
                }
                if self.children[0].name != "TEXT" {
                    Render::Element(
                        mouse_area(greedy_render(&self.children))
                            .on_release(Message::new(MessageKind::LinkOpened(url), None))
                        .into(), true
                    )
                } else {
                    Render::Span(
                        span(unescape_html_text(greedy_text(&self.children).as_str()))
                            .link(Message::new(MessageKind::LinkOpened(url), None))
                            .color(color!(0x0000fa))
                    )
                }

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
            "table" => {
                let mut rows = vec![];

                for child in &self.children {
                    match child.name.as_str() {
                        "thead" => {
                            // Table head

                            let mut row = Vec::<Element>::new();

                            // Iterate through columns in the table head
                            for cell in &child.children[0].children {
                                let output = cell.view();
                                match output {
                                    Render::Element(e, _) => row.push(e),
                                    Render::Span(s) => row.push(rich_text([s]).into()),
                                }
                            }

                            rows.push(grid_row(row))
                        }
                        "tbody" => {
                            // Table body
                            let mut row = Vec::<Element>::new();
                            for trow in &child.children {
                                for cell in &trow.children {
                                    let output = cell.view();
                                    match output {
                                        Render::Element(e, _) => row.push(e),
                                        Render::Span(s) => row.push(rich_text([s]).into()),
                                    }
                                }
                            }
                            rows.push(grid_row(row))
                        }
                        x => {
                            // invalid:
                            error!("Invalid table element '{}' - Cannot render", x)
                        }
                    }
                }



                Render::Element(
                    container(
                        grid(rows)
                    ).into(),
                    false
                )
            }
            "th" | "td" => {
                if self.attributes.contains_key("style") {
                    let mut alignment = Horizontal::Left;
                    let align = self.attributes.get("style").unwrap().clone().unwrap();
                    match align.as_str() {
                        "text-align: center" => alignment = Horizontal::Center,
                        "text-align: right" => alignment = Horizontal::Right,
                        _ => {}
                    }

                    Render::Element(greedy_render_aligned(&self.children, alignment), false)
                } else {
                    Render::Element(greedy_render(&self.children), false)
                }
            },
            "img" => {
                let maybe_src = self.attributes.get("src").unwrap();
                if let Some(src) = maybe_src {
                    let hash = hash_str(src);
                    warn!("Image handling support is experimental - Asset '{}' -> '{}'", src, hash);
                    // let mut alt = self.attributes.get("alt").unwrap_or(&None).clone().unwrap_or("".to_string());
                    // let asset = format!()
                    // let can_render = std::fs::exists()


                    // if alt.len() > 0 {
                    //
                    // } else {
                    //
                    // }

                    let maybe_buffer = reqwest::blocking::get(src);

                    if let Ok(response) = maybe_buffer {
                        Render::Element(Svg::new(Handle::from_memory(Cow::Owned(response.bytes().unwrap().to_vec()))).height(Length::Shrink).width(Length::Shrink).into(), true)
                    } else {
                        Render::Span("Image not found".into())
                    }



                } else {
                    Render::Span("Image not found".into())
                }
            }
            _ => {
                // dbg!(&self);
                Render::Element(rich_text([span(format!("{} - Not Supported", self.name))]).into(), false)
            }
        }
    }
}


fn unescape_html_text(src: &str) -> String {
    let after: String = html_escape::decode_html_entities(src).into();
    after.replace("\t", " ")
}


fn greedy_render_aligned(elements: &[ElWrapper], align: iced::alignment::Horizontal) -> Element {
    let mut col = vec![];
    let mut spans = vec![];
    let mut inline_elems = vec![];

    for element in elements {
        match element.view() {
            Render::Element(e, inline) => {
                col.push(rich_text(spans).align_x(align).into());
                spans = vec![];
                if inline {
                    inline_elems.push(e);
                } else {
                    col.push(row(inline_elems).width(Length::Shrink).wrap().into());
                    inline_elems = vec![];
                    col.push(e);
                }
            }
            Render::Span(s) => spans.push(s.into()),
        }
    }
    col.push(rich_text(spans).align_x(align).into());
    col.push(row(inline_elems).width(Length::Shrink).wrap().into());


    column(
        col
    ).into()
}

fn greedy_render(elements: &[ElWrapper]) -> Element {
    let mut col = vec![];
    let mut spans = vec![];
    let mut inline_elems = vec![];

    for element in elements {
        match element.view() {
            Render::Element(e, inline) => {
                col.push(rich_text(spans).into());
                spans = vec![];
                if inline {
                    inline_elems.push(e);
                } else {
                    col.push(row(inline_elems).width(Length::Shrink).wrap().into());
                    inline_elems = vec![];
                    col.push(e);
                }
            }
            Render::Span(s) => spans.push(s.into()),
        }
    }
    col.push(rich_text(spans).into());
    col.push(row(inline_elems).width(Length::Shrink).wrap().into());


    column(
        col
    ).into()
}

fn greedy_text(elements: &[ElWrapper]) -> String {
    elements.iter().map(|c| unescape_html_text(c.display_text.as_str()))
        .collect::<Vec<String>>()
        .join(" ")
}


fn heading<'a>(text: String, size: f32) -> Render<'a> {
    Render::Element(
        rich_text([
            span(text)
                .font(FONT_BOLD)
                .size(size)
                .into(),
        ]).into(),
        false
    )
}
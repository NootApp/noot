use std::path::PathBuf;
use std::collections::HashMap;
use url::Url;
use html_parser::{Dom, Node};
use pulldown_cmark::{Parser, Options, Event};
use iced::{color, Border, Length, Padding};
use iced::border::Radius;
use iced::widget::{row, column, span, container, horizontal_rule, mouse_area};
use iced::widget::text::Span;
use iced_aw::{grid, grid_row};
use iced_core::alignment::Horizontal;
use iced_core::Font;
use iced_core::font::{Family, Style, Weight};
use iced_core::font::Weight::{Bold, Medium, Normal};
use crate::consts::*;
use crate::utils::components::widgets::rich_text;
use crate::runtime::{Element, GLOBAL_STATE};
use crate::runtime::Message;
use crate::runtime::messaging::MessageKind;
use crate::utils::components::widgets::rich_text::Rich;
use crate::utils::cryptography::hashing::hash_str;

#[derive(Debug, Clone)]
pub struct Buffer {
    pub id: String,
    pub name: String,
    pub url: Url,
    pub doc: Vec<ElWrapper>,
    pub tts_segments: Vec<String>
    // assets: BTreeMap<String, Vec<u8>>
}

pub enum Image {
    Svg(Vec<u8>),
    Other(Vec<u8>)
}


impl Buffer {
    pub fn new<U: Into<String> + Clone + std::fmt::Debug>(name: String, workspace: String, url: U, content: String) -> Self {
        let dom = Dom::parse(&content).unwrap();
        let mut els = vec![];
        let mut root_dir = PathBuf::from(url.clone().into());
        root_dir.pop();

        for element in dom.children {
            match element {
                Node::Comment(_) => continue,
                Node::Text(_) => els.push(ElWrapper::new(element, workspace.clone())),
                Node::Element(_) => els.push(ElWrapper::new(element, workspace.clone()))
            }
        }

        // let asset_base = PathBuf::from(wsm.source.disk_path).with_file_name("assets");

        Self {
            id: nanoid!(5),
            name,
            url: Url::parse(&url.into()).unwrap(),
            doc: els,
            tts_segments: vec![]
            // assets: BTreeMap::new()
        }
    }

    pub fn from_md<U: Into<String> + std::fmt::Debug + Clone>(name: String, workspace: String, url: U, content: String) -> Self {
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

        Self::new(name, workspace, url, html_output)
    }

    pub fn view(&self) -> Element {
        let start = std::time::Instant::now();
        let (v, _texts) = greedy_render(&self.doc);

        let view = container(
            v
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
    pub name: String,
    pub attributes: HashMap<String, Option<String>>,
    // classes: Vec<String>,
    pub children: Vec<ElWrapper>,
    pub display_text: String
}


pub enum Render<'a> {
    /// Contains an element, and a boolean stating whether the widget can be inlined or not
    Element(Element<'a>, bool),
    /// Contains a rich text span
    Span(Span<'a, Message>)
}


impl ElWrapper {
    pub fn new(el: Node, workspace: String) -> Self {

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
                    children.push(Self::new(child, workspace.clone()))
                }

                let mut s = Self {
                    // id: element.id.clone(),
                    name: element.name.clone(),
                    attributes: element.attributes.clone(),
                    // classes: element.classes.clone(),
                    children,
                    display_text: String::new(),
                };

                if s.name == "img" {
                    let src = s.attributes.get("src").unwrap().clone().unwrap();
                    let hash = hash_str(src);
                    let uri = Url::parse(&format!("noot://{}/assets/{}", workspace, hash)).unwrap();
                    s.attributes.insert("cached-src".to_string(), Some(uri.to_string()));
                }

                s
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

    pub fn view(&self, section_text: Option<String>) -> (Render, String) {
        match self.name.as_str() {
            "TEXT" =>
                (Render::Span(a11_span(unescape_html_text(self.display_text.as_str()))), unescape_html_text(self.display_text.as_str())),
            "hr" => (Render::Element(horizontal_rule(2).into(), false), "".to_string()),
            "h1" => (heading(greedy_text(&self.children), HEADER_SIZE_1, section_text.clone().unwrap_or(greedy_text(&self.children))), section_text.unwrap_or(greedy_text(&self.children))),
            "h2" => (heading(greedy_text(&self.children), HEADER_SIZE_2, greedy_text(&self.children)), greedy_text(&self.children)),
            "h3" => (heading(greedy_text(&self.children), HEADER_SIZE_3, greedy_text(&self.children)), greedy_text(&self.children)),
            "h4" => (heading(greedy_text(&self.children), HEADER_SIZE_4, greedy_text(&self.children)), greedy_text(&self.children)),
            "h5" => (heading(greedy_text(&self.children), HEADER_SIZE_5, greedy_text(&self.children)), greedy_text(&self.children)),
            "p" => {
                let (v, t) = greedy_render(&self.children);
                (Render::Element(v, false), t.join(" "))
            }
            "div" => {
                let (v, t) = greedy_render(&self.children);
                (Render::Element(v, false), t.join(" "))
            },
            "pre" => {
                (
                    Render::Element(
                        container(
                            rich_text(
                                self.children[0].children.iter().map(|c| {
                                    a11_mono(unescape_html_text(c.display_text.as_str()))
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
                    ),
                    greedy_text(&self.children)
                )

            }
            "code" => {
                let mut padding = Padding::new(2.);
                padding.left = 3.;
                padding.right = 3.;

                // All children should be text elements, so we can safely render as text
                (
                    Render::Span(
                        a11_mono(
                            self.children.iter().map(|c| unescape_html_text(c.display_text.as_str()))
                                .collect::<Vec<String>>()
                                .join(" ")
                        )
                            .background(color!(0x1a1a1a))
                            .border(Border {
                                color: color!(0x2a2a2a),
                                width: 2.0,
                                radius: Radius::new(5),
                            })
                            .padding(padding)
                    ),
                    greedy_text(&self.children)
                )
            },
            "li" => {
                (
                    Render::Span(
                        a11_span(format!("- {}", greedy_text(&self.children)))
                    ),
                    greedy_text(&self.children)
                )
            },
            "ul" => {
                (
                    Render::Element(
                        column(
                            self.children.iter().map(|c| {
                                row!(a11_text(format!("- {}", greedy_text(&c.children)))).wrap().into()
                            })
                        ).into(),
                        false
                    ),
                    greedy_text(&self.children)
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
                    let (v, t) = greedy_render(&self.children);
                    (
                        Render::Element(
                            mouse_area(v)
                                .on_release(Message::new(MessageKind::LinkOpened(url), None))
                            .into(), true
                        ),
                        t.join(" ")
                    )
                } else {
                    (
                        Render::Span(
                            a11_span(unescape_html_text(greedy_text(&self.children).as_str()))
                                .link(Message::new(MessageKind::LinkOpened(url), None))
                                .color(color!(0x0000fa))
                        ),
                        greedy_text(&self.children)
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
                                let output = cell.view(None).0;
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
                                    let output = cell.view(None).0;
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



                (
                    Render::Element(
                        container(
                            grid(rows)
                        ).into(),
                        false
                    ),
                    "Table containing content which is not supported for screen readers".to_string()
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

                    (
                        Render::Element(greedy_render_aligned(&self.children, alignment).0, false),
                        greedy_text(&self.children)
                    )
                } else {
                    let (v, t) = greedy_render(&self.children);
                    (
                        Render::Element(v, false),
                        t.join(" ")
                    )
                }
            },
            "img" => {
                dbg!(&self.attributes);
                // let maybe_src = self.attributes.get("src").unwrap();
            //     if let Some(src) = maybe_src {
            //         // let hash = hash_str(src);
            //         // warn!("Image handling support is experimental - Asset '{}' -> '{}'", src, hash);
            //         // Render::Element(Svg::new(Handle::from_path().height(Length::Shrink).width(Length::Shrink).into(), true));
            //         let mut alt = self.attributes.get("alt").unwrap_or(&None).clone().unwrap_or("".to_string());
            //         // let asset = format!()
            //         // let can_render = std::fs::exists()
            //
            //
            //         // if alt.len() > 0 {
            //         //
            //         // } else {
            //         //
            //         // }
            //
            //         // let maybe_buffer = reqwest::blocking::get(src);
            //
            // //         if let Ok(response) = maybe_buffer {
            // //             Render::Element(Svg::new(Handle::from_memory(Cow::Owned(response.bytes().unwrap().to_vec()))).height(Length::Shrink).width(Length::Shrink).into(), true)
            // //         } else {
            //             (Render::Span(a11_span("Image not found")), alt)
            // //         }
            // //
            // //
            // //
            //     } else {
                    (Render::Span(a11_span("Image not found")), "Image not found".into())
                // }
            }
            _ => {
                // dbg!(&self);
                (
                    Render::Element(rich_text([span(format!("{} - Not Supported", self.name))]).into(), false),
                    greedy_text(&self.children)
                )
            }
        }
    }
}


fn unescape_html_text(src: &str) -> String {
    let after: String = html_escape::decode_html_entities(src).into();
    after.replace("\t", " ")
}


fn greedy_render_aligned(elements: &[ElWrapper], align: Horizontal) -> (Element, Vec<String>) {
    let mut col = vec![];
    let mut spans = vec![];
    let mut inline_elems = vec![];
    let mut text_elems = vec![];

    for element in elements {
        let (view, tts) = element.view(None);
        match view {
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
        text_elems.push(tts);
    }
    col.push(rich_text(spans).align_x(align).into());
    col.push(row(inline_elems).width(Length::Shrink).wrap().into());



    (
        column(
            col
        ).into(),
        text_elems
    )
}

fn greedy_render(elements: &[ElWrapper]) -> (Element, Vec<String>) {
    let mut col = vec![];
    let mut spans = vec![];
    let mut inline_elems = vec![];
    let mut text_elems = vec![];
    let mut text_since_h1 = String::new();
    let mut last_header_index = 0;
    let mut index = 0;

    for element in elements {
        let (view, tts) = element.view(None);

        if element.name == "h1" && col.len() > 0 {
            let (v, _) = element.view(Some(text_since_h1));
            col[last_header_index+1] = match v {
                Render::Element(e, _) => e.into(),
                _ => unreachable!(),
            };
            text_since_h1 = String::new();
            last_header_index = index;
        }

        text_since_h1 = format!("{} {}", text_since_h1, tts);
        text_elems.push(tts);

        match view {
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
        // dbg!(&text_since_h1, &last_header_index);
        index +=1;
    }
    col.push(rich_text(spans).into());
    col.push(row(inline_elems).width(Length::Shrink).wrap().into());


    (
        column(
            col
        ).into(),
        text_elems
    )
}

fn greedy_text(elements: &[ElWrapper]) -> String {
    elements.iter().map(|c| unescape_html_text(c.display_text.as_str()))
        .collect::<Vec<String>>()
        .join(" ")
}


fn heading<'a>(text: String, size: f32, tts: String) -> Render<'a> {
    // dbg!(&text, size, &tts);
    Render::Element(
        // row!(
        //     tooltip(
        //         RichButton::new_with_icon(Icon::PlayArrow, "").width(40.).on_press(Message::say(tts.clone())),
        //         "Listen to this section",
        //         Position::Top
        //     ),
        //     horizontal_space().width(10.),
            a11_heading(text, size).width(Length::Fill).into(),
        // ).width(Length::Fill).into(),
        false
    )
}


fn a11_mono<'a, T: Into<String>>(t: T) -> Span<'a, Message> {
    let font_name = get_a11_font();

    span(t.into())
        .font(build_font(font_name, Normal))
}

fn a11_span<'a, T: Into<String>>(t: T) -> Span<'a, Message> {
    let font_name = get_a11_font();

    span(t.into())
        .font(build_font(font_name, Normal))
}

fn a11_text<'a, T: Into<String>>(t: T) -> Rich<'a, Message> {
    let font_name = get_a11_font();

    rich_text([
        span(t.into())
            .font(build_font(font_name, Normal))
    ])
}

fn a11_heading<'a, T: Into<String>>(t: T, size: f32) -> Rich<'a, Message> {
    let font_name = get_a11_font();

    rich_text([
        span(t.into())
            .font(build_font(font_name, Bold))
            .size(size)
    ])
}

fn get_a11_font() -> String {
    if GLOBAL_STATE.lock().unwrap().store.get_setting::<bool>("appearance.font.dyslexic.enable").unwrap().value == true {
        GLOBAL_STATE.lock().unwrap().store.get_setting::<String>("appearance.font.dyslexic.primary").unwrap().value
    } else {
        GLOBAL_STATE.lock().unwrap().store.get_setting::<String>("appearance.font.primary").unwrap().value
    }
}

fn build_font(name: String, weight: Weight) -> Font {
    info!("Font name: {}", name);
    Font {
        family: Family::Name(name.leak()),
        weight,
        stretch: Default::default(),
        style: Style::Normal,
    }
}
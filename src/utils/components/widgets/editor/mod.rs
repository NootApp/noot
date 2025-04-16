use iced::Renderer;
use iced_core::{Layout, Length, Pixels, Point, Rectangle, Size, Theme, Widget};
use iced_core::layout::{Limits, Node};
use iced_core::renderer::Style;
use iced_core::widget::Tree;
use iced_core::widget::text::Catalog;
use crate::runtime::Message;
use crate::storage::workspace::buffer::Buffer;

pub struct Cursor {
    line: usize,
    column: usize,
}

impl Cursor {
    pub fn new(line: usize, column: usize) -> Self {
        Cursor { line, column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn line_and_column(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}

pub struct MarkdownEditor<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Message: Clone + 'static,
    Theme: Catalog,
    Renderer: iced_core::text::Renderer,
{
    lines: Vec<&'static str>,
    mode: EditorMode,
    render_buffer: Buffer,
    size: Option<Pixels>,
    line_height: f32,
    width: Length,
    height: Length,
    font: Option<Renderer::Font>,
    cursors: Vec<Cursor>,
}

pub struct State {
    lines: Vec<&'static str>,
    mode: EditorMode,
    cursors: Vec<Cursor>,
}

pub enum EditorMode {
    /// Editor section only, raw Markdown, nothing else
    Edit,

    /// What You See Is What You Get. Think Microsoft Word, but for Markdown
    WYSIWYG,

    /// Self-explanatory, show the Edit view on one side, and the Render view on the other
    SideBySide,

    /// Show only the rendered Markdown content, no editing enabled
    Render
}

impl Widget<Message, Theme, Renderer> for MarkdownEditor<Message, Theme, Renderer> {
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        layout(tree.state.downcast_mut::<State<Message, Renderer::>>())
    }

    fn draw(&self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout<'_>, cursor: Cursor, viewport: &Rectangle) {
        todo!()
    }
}
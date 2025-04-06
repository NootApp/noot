use iced_core::widget::text::Catalog;

pub mod status_bar;
pub mod rich_text;

pub fn rich_text<'a, Link, Theme, Renderer>(
    spans: impl AsRef<[iced_core::text::Span<'a, Link, Renderer::Font>]> + 'a,
) -> rich_text::Rich<'a, Link, Theme, Renderer>
where
    Link: Clone + 'static,
    Theme: Catalog + 'a,
    Renderer: iced::advanced::graphics::core::text::Renderer,
    Renderer::Font: 'a,
{
    rich_text::Rich::with_spans(spans)
}
use std::sync::Arc;
use iced::{color, Background, Border, Color, Element, Length, Shadow};
use iced::gradient::Gradient;
use iced::border::Radius;
use iced::widget::{button, center, container, horizontal_space, row};
use iced::window::Id;
use iced_core::{border, event, gradient, layout, mouse, overlay, renderer, touch, Clipboard, Event, Layout, Padding, Rectangle, Shell, Size, Theme, Vector, Widget};
use iced_core::theme::palette;
use iced_core::widget::{tree, Operation, Tree};
use material_icons::Icon;
use crate::consts::{BUTTON_DEFAULT_BACKGROUND, BUTTON_DEFAULT_TEXT, FONT_ICON};
use crate::runtime::messaging::Message;
use crate::runtime::Task;
use crate::utils::components::{ComponentMessage, ComponentRegistration, COMPONENT_TRACKER};
use crate::utils::components::icons::Icon as NootIcon;

#[derive(Clone, Debug)]
pub enum ButtonMessageKind {
    MouseOver(String),
    MouseOut(String),
    Clicked(String),
}

#[derive(Clone, Debug)]
pub struct ButtonMessage {
    pub kind: ButtonMessageKind,
    pub window_id: Id,
}

impl Into<Message> for ButtonMessage {
    fn into(self) -> Message {
        ComponentMessage::button(self).into()
    }
}

impl ButtonMessage {
    pub fn new(kind: ButtonMessageKind, window_id: Id) -> Self {
        ButtonMessage { kind, window_id }
    }

    fn from_reg(reg: Arc<ComponentRegistration>) -> Self {
        Self::new(ButtonMessageKind::Clicked(reg.component_id.clone()), reg.window_id)
    }

    pub fn click(reg: Arc<ComponentRegistration>) -> Self {
        Self::from_reg(reg)
    }

    pub fn mouse_out(reg: Arc<ComponentRegistration>) -> Self {
        Self::from_reg(reg)
    }

    pub fn mouse_over(reg: Arc<ComponentRegistration>) -> Self {
        Self::from_reg(reg)
    }
}

pub struct Button<T: Into<Message>> {
    registration: Arc<ComponentRegistration>,
    on_click: Option<fn() -> T>,
    disabled: bool,
    hovered: bool,
    text: String,
    icon: Option<NootIcon>
}

impl Button<Message> {
    pub fn new<T: Into<String>>(text: T, id: Id) -> Button<Message> {
        let reg = COMPONENT_TRACKER.lock().unwrap().register(id);
        Self {
            registration: reg,
            on_click: None,
            disabled: false,
            hovered: false,
            text: text.into(),
            icon: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, on_click: fn() -> Message) -> Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn update(&mut self, message: ButtonMessage) -> Task {
        match message.kind {
            ButtonMessageKind::MouseOver(text) => {
                if self.registration.component_id == text {
                    self.hovered = true;
                }
                Task::none()
            },
            ButtonMessageKind::MouseOut(text) => {
                if self.registration.component_id == text {
                    self.hovered = false;
                }
                Task::none()
            },
            ButtonMessageKind::Clicked(text) => {
                if self.registration.component_id == text {
                    return match self.on_click {
                        Some(on_click) => Task::done(on_click()),
                        _ => Task::none()
                    }
                }
                Task::none()
            }
        }


    }

    pub fn view<T>(&self) -> Element<'_, Message> {
        match &self.icon {
            Some(icon) => {

                button(row!(icon.view(), iced::widget::text(self.text.clone())))
                    .into()
            },
            None => {

                button(iced::widget::text(self.text.clone()))
                    .into()
            }
        }

    }
}


pub struct ButtonStyle {
    background: Option<Background>,
    color: Option<Color>,
    border: Option<Border>,
    shadow: Option<Shadow>,
}

impl ButtonStyle {
    pub fn new() -> Self {
        ButtonStyle {
            background: Some(Background::Color(color!(BUTTON_DEFAULT_BACKGROUND))),
            color: None,
            border: None,
            shadow: None,
        }
    }
    pub fn with_background_color(&mut self, color: i32) -> &mut Self {
        self.background = Some(Background::Color(color!(color)));
        self
    }

    pub fn with_background(&mut self, bg: Background) -> &mut Self {
        self.background = Some(bg);
        self
    }

    pub fn with_color(&mut self, color: i32) -> &mut Self {
        self.color = Some(color!(color));
        self
    }

    pub fn with_border(&mut self, border: Border) -> &mut Self {
        self.border = Some(border);
        self
    }

    pub fn with_shadow(&mut self, shadow: Shadow) -> &mut Self {
        self.shadow = Some(shadow);
        self
    }

    pub fn compile(&self) -> button::Style {
        button_style(self.background, self.color, self.border, self.shadow)
    }
}

pub fn button_style(background: Option<Background>, color: Option<Color>, border: Option<Border>, shadow: Option<Shadow>) -> button::Style {
    let background = Some(background.unwrap_or(Background::Color(color!(BUTTON_DEFAULT_BACKGROUND))));
    let text_color = color.unwrap_or(color!(BUTTON_DEFAULT_TEXT));
    let border = border.unwrap_or(Border {
        color: color!(0x1a1a1a),
        width: 1.,
        radius: Radius::new(5.),
    });
    let shadow = shadow.unwrap_or_default();

    button::Style {
        background,
        text_color,
        border,
        shadow,
    }
}


pub fn button_with_icon<'a, T: Into<String>>(icon: material_icons::Icon, message: T) -> iced::widget::Button<'a, Message> {
    button(
        center(
            row!(
                iced::widget::text(material_icons::icon_to_char(icon)).font(FONT_ICON).size(18),
                horizontal_space().width(5.),
                iced::widget::text(message.into()).size(16)
            )
        ).height(Length::Shrink).width(Length::Fill)
    ).style(|_,_| ButtonStyle::new().compile().into())
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
    is_hovered: bool,
}

pub(crate) const DEFAULT_PADDING: Padding = Padding {
    top: 5.0,
    bottom: 5.0,
    right: 10.0,
    left: 10.0,
};

#[allow(missing_debug_implementations)]
pub struct RichButton<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced_core::Renderer,
    Theme: Catalog
{
    content: Element<'a, Message, Theme, Renderer>,
    on_press: Option<OnPress<'a, Message>>,
    on_hover: Option<OnPress<'a, Message>>,
    width: Length,
    height: Length,
    padding: Padding,
    clip: bool,
    class: Theme::Class<'a>
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>)
}

impl <'a, Message: Clone> OnPress<'a, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(msg) => msg.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

impl<'a, Message: 'a, Theme, Renderer> RichButton<'a, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer + iced_core::text::Renderer + 'a,
    Theme: Catalog + iced::widget::text::Catalog + container::Catalog + 'a,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        RichButton {
            content,
            on_press: None,
            on_hover: None,
            width: size.width.fluid(),
            height: size.height.fluid(),
            padding: DEFAULT_PADDING,
            clip: false,
            class: <Theme as Catalog>::default(),
        }
    }

    pub fn new_with_icon(icon: Icon, content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self where <Renderer as iced_core::text::Renderer>::Font: From<iced::Font> {
        let content = center(
            row!(
                iced::widget::text(material_icons::icon_to_char(icon)).font(FONT_ICON).size(18),
                horizontal_space().width(5.),
                content.into()
            )
        ).height(Length::Shrink).width(Length::Fill);

        Self::new(content)
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }

    /// Sets the message that will be produced when the [`RichButton`] is pressed,
    /// if `Some`.
    ///
    /// If `None`, the [`RichButton`] will be disabled.
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press.map(OnPress::Direct);
        self
    }


    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    pub fn class(mut self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
for RichButton<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_core::Renderer,
    Theme: Catalog
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::padded(
            limits,
            self.width,
            self.height,
            self.padding,
            |limits| {
                self.content.as_widget().layout(
                    &mut tree.children[0],
                    renderer,
                    limits,
                )
            },
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();
        let is_mouse_over = cursor.is_over(bounds);

        let status = if self.on_press.is_none() {
            Status::Disabled
        } else if is_mouse_over {
            let state = tree.state.downcast_ref::<State>();

            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        let style = theme.style(&self.class, status);

        if style.background.is_some()
            || style.border.width > 0.0
            || style.shadow.color.a > 0.0
        {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    shadow: style.shadow,
                },
                style
                    .background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        let viewport = if self.clip {
            bounds.intersection(viewport).unwrap_or(*viewport)
        } else {
            *viewport
        };

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: style.text_color,
            },
            content_layout,
            cursor,
            &viewport,
        );
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            return event::Status::Captured;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        let state = tree.state.downcast_mut::<State>();

                        state.is_pressed = true;

                        return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = self.on_press.as_ref().map(OnPress::get)
                {
                    let state = tree.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press);
                        }

                        return event::Status::Captured;
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let state = tree.state.downcast_mut::<State>();
                if cursor.is_over(layout.bounds()) && !state.is_hovered {
                    state.is_hovered = true;
                } else if state.is_hovered {
                    state.is_hovered = false;
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over && self.on_press.is_some() {
            mouse::Interaction::Pointer
        } else if is_mouse_over && self.on_press.is_none() {
            mouse::Interaction::NotAllowed
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<RichButton<'a, Message, Theme, Renderer>>
for iced_core::Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(button: RichButton<'a, Message, Theme, Renderer>) -> Self {
        Self::new(button)
    }
}


/// The possible status of a [`iced::widget::Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`iced::widget::Button`] can be pressed.
    Active,
    /// The [`iced::widget::Button`] can be pressed and it is being hovered.
    Hovered,
    /// The [`iced::widget::Button`] is being pressed.
    Pressed,
    /// The [`iced::widget::Button`] cannot be pressed.
    Disabled,
}

/// The style of a button.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`Background`] of the button.
    pub background: Option<Background>,
    /// The text [`Color`] of the button.
    pub text_color: Color,
    /// The [`Border`] of the buton.
    pub border: Border,
    /// The [`Shadow`] of the butoon.
    pub shadow: Shadow,
}

impl Style {
    /// Updates the [`Style`] with the given [`Background`].
    pub fn with_background(self, background: impl Into<Background>) -> Self {
        Self {
            background: Some(background.into()),
            ..self
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: None,
            text_color: Color::BLACK,
            border: Border::default(),
            shadow: Shadow::default(),
        }
    }
}

/// The theme catalog of a [`iced::widget::Button`].
pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`iced::widget::Button`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// A primary button; denoting a main action.
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.primary.strong);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.primary.base.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A secondary button; denoting a complementary action.
pub fn secondary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.secondary.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.secondary.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A success button; denoting a good outcome.
pub fn success(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.success.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.success.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A danger button; denoting a destructive action.
pub fn danger(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(palette.danger.base);

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            background: Some(Background::Color(palette.danger.strong.color)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

/// A text button; useful for links.
pub fn text(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let base = Style {
        text_color: palette.background.base.text,
        ..Style::default()
    };

    match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
            text_color: palette.background.base.text.scale_alpha(0.8),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn styled(pair: palette::Pair) -> Style {
    Style {
        background: Some(Background::Color(pair.color)),
        text_color: pair.text,
        border: border::rounded(2),
        ..Style::default()
    }
}

fn disabled(style: Style) -> Style {
    Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}



lazy_static!(
    pub static ref NEON_BUTTON_CONTAINER: container::Style = container::Style {
        text_color: None,
        background: Some(
            Background::Gradient(
                Gradient::Linear(
                    gradient::Linear::new(45)
                        .add_stop(0.0, color!(0x00d4ff))
                        .add_stop(0.25, color!(0x3f3fb9))
                        .add_stop(0.5, color!(0x020024))
                        .add_stop(0.75, color!(0x3f3fb9))
                        .add_stop(1.0, color!(0x00d4ff))
                )
            )
        ),
        border: Border::default().rounded(5),
        ..container::Style::default()
    };
);
use iced::advanced::widget::{tree, Tree};
use iced::advanced::{layout, mouse, overlay, Layout, Widget};
use iced::widget::Column;
use iced::{alignment, event, Element, Event, Length, Padding, Pixels, Rectangle, Size, Vector};
use n16_theme::Base16Theme;

/// A thin wrapper around `Column` that lets it be translated in the draw phase
pub struct ScrolledColumn<'a, Message, Theme = Base16Theme, Renderer = iced::Renderer> {
  inner: Column<'a, Message, Theme, Renderer>,
  height: Length,
  view_child: usize,
}

impl<'a, Message, Theme, Renderer> ScrolledColumn<'a, Message, Theme, Renderer>
where
  Renderer: iced::advanced::Renderer,
{
  /// Creates an empty [`ScrolledColumn`].
  pub fn new() -> Self {
    Self::from_vec(Vec::new())
  }

  /// Creates a [`ScrolledColumn`] with the given capacity.
  pub fn with_capacity(capacity: usize) -> Self {
    Self::from_vec(Vec::with_capacity(capacity))
  }

  /// Creates a [`ScrolledColumn`] with the given elements.
  pub fn with_children(
    children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
  ) -> Self {
    let iterator = children.into_iter();

    Self::with_capacity(iterator.size_hint().0).extend(iterator)
  }

  /// Creates a [`ScrolledColumn`] from an already allocated [`Vec`].
  ///
  /// Keep in mind that the [`ScrolledColumn`] will not inspect the [`Vec`], which means
  /// it won't automatically adapt to the sizing strategy of its contents.
  ///
  /// If any of the children have a [`Length::Fill`] strategy, you will need to
  /// call [`ScrolledColumn::width`] or [`ScrolledColumn::height`] accordingly.
  pub fn from_vec(children: Vec<Element<'a, Message, Theme, Renderer>>) -> Self {
    Self {
      inner: Column::from_vec(children),
      height: Length::Shrink,
      view_child: 5,
    }
    .validate()
  }

  fn validate(self) -> Self {
    let size_hint = self.inner.size_hint();

    debug_assert!(
      !size_hint.height.is_fill(),
      "scrolled_column content must not fill its vertical scrolling axis"
    );

    self
  }

  /// Sets the vertical spacing _between_ elements.
  ///
  /// Custom margins per element do not exist in iced. You should use this
  /// method instead! While less flexible, it helps you keep spacing between
  /// elements consistent.
  pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
    self.inner = self.inner.spacing(amount);
    self
  }

  /// Sets the [`Padding`] of the [`ScrolledColumn`].
  pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
    self.inner = self.inner.padding(padding);
    self
  }

  /// Sets the width of the [`ScrolledColumn`].
  pub fn width(mut self, width: impl Into<Length>) -> Self {
    self.inner = self.inner.width(width);
    self
  }

  /// Sets the height of the [`ScrolledColumn`].
  pub fn height(mut self, height: impl Into<Length>) -> Self {
    self.height = height.into();
    self
  }

  /// Sets the maximum width of the [`ScrolledColumn`].
  pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
    self.inner = self.inner.max_width(max_width);
    self
  }

  /// Sets the horizontal alignment of the contents of the [`ScrolledColumn`] .
  pub fn align_x(mut self, align: impl Into<alignment::Horizontal>) -> Self {
    self.inner = self.inner.align_x(align);
    self
  }

  /// Sets whether the contents of the [`ScrolledColumn`] should be clipped on
  /// overflow.
  pub fn clip(mut self, clip: bool) -> Self {
    self.inner = self.inner.clip(clip);
    self
  }

  /// Adds an element to the [`ScrolledColumn`].
  pub fn push(mut self, child: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
    self.inner = self.inner.push(child);
    self
  }

  /// Adds an element to the [`ScrolledColumn`], if `Some`.
  pub fn push_maybe(self, child: Option<impl Into<Element<'a, Message, Theme, Renderer>>>) -> Self {
    if let Some(child) = child {
      self.push(child)
    } else {
      self
    }
  }

  /// Extends the [`ScrolledColumn`] with the given children.
  pub fn extend(
    self,
    children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
  ) -> Self {
    children.into_iter().fold(self, Self::push)
  }

  pub fn view_child(mut self, view_child: usize) -> Self {
    self.view_child = view_child;
    self
  }
}

impl<'a, Message, Renderer> Default for ScrolledColumn<'a, Message, Renderer>
where
  Renderer: iced::advanced::Renderer,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<'a, Message, Theme, Renderer: iced::advanced::Renderer>
  FromIterator<Element<'a, Message, Theme, Renderer>>
  for ScrolledColumn<'a, Message, Theme, Renderer>
{
  fn from_iter<T: IntoIterator<Item = Element<'a, Message, Theme, Renderer>>>(iter: T) -> Self {
    Self::with_children(iter)
  }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
  for ScrolledColumn<'a, Message, Theme, Renderer>
where
  Renderer: iced::advanced::Renderer,
{
  fn tag(&self) -> tree::Tag {
    tree::Tag::of::<State>()
  }

  fn state(&self) -> tree::State {
    tree::State::new(State::new())
  }

  fn children(&self) -> Vec<Tree> {
    self.inner.children()
  }

  fn diff(&self, tree: &mut Tree) {
    self.inner.diff(tree)
  }

  fn size(&self) -> iced::Size<iced::Length> {
    Size {
      height: self.height,
      width: self.inner.size().width,
    }
  }

  fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
    let inner_limits = layout::Limits::new(
      Size::new(limits.min().width, limits.min().height),
      Size::new(limits.max().width, f32::MAX),
    );

    let size = self.size();
    let node = layout::Node::with_children(
      limits.resolve(size.width, size.height, Size::ZERO),
      vec![self.inner.layout(tree, renderer, &inner_limits)],
    );

    let layout = Layout::new(&node);

    let bounds = layout.bounds();
    let content_layout = layout.children().next().unwrap();

    let state = tree.state.downcast_mut::<State>();
    state.scroll_child_into_view(bounds, content_layout, self.view_child);

    node
  }

  fn operate(
    &self,
    tree: &mut Tree,
    layout: Layout,
    renderer: &Renderer,
    operation: &mut dyn iced::advanced::widget::Operation,
  ) {
    let content_layout = layout.children().next().unwrap();

    self
      .inner
      .operate(tree, content_layout, renderer, operation);
  }

  fn on_event(
    &mut self,
    tree: &mut Tree,
    event: Event,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    renderer: &Renderer,
    clipboard: &mut dyn iced::advanced::Clipboard,
    shell: &mut iced::advanced::Shell<'_, Message>,
    _viewport: &Rectangle,
  ) -> event::Status {
    let state = tree.state.downcast_ref::<State>();
    let translation = state.translation;

    let bounds = layout.bounds();
    let content_layout = layout.children().next().unwrap();

    let cursor = match cursor.position_over(bounds) {
      Some(cursor_position) => mouse::Cursor::Available(cursor_position + translation),
      _ => mouse::Cursor::Unavailable,
    };

    self.inner.on_event(
      tree,
      event,
      content_layout,
      cursor,
      renderer,
      clipboard,
      shell,
      &Rectangle {
        y: bounds.y + translation.y,
        x: bounds.x + translation.x,
        ..bounds
      },
    )
  }

  fn mouse_interaction(
    &self,
    tree: &Tree,
    layout: Layout,
    cursor: mouse::Cursor,
    _viewport: &Rectangle,
    renderer: &Renderer,
  ) -> mouse::Interaction {
    let state = tree.state.downcast_ref::<State>();
    let translation = state.translation;

    let bounds = layout.bounds();
    let content_layout = layout.children().next().unwrap();

    self.inner.mouse_interaction(
      tree,
      content_layout,
      cursor,
      &Rectangle {
        y: bounds.y + translation.y,
        x: bounds.x + translation.x,
        ..bounds
      },
      renderer,
    )
  }

  fn draw(
    &self,
    tree: &Tree,
    renderer: &mut Renderer,
    theme: &Theme,
    style: &iced::advanced::renderer::Style,
    layout: iced::advanced::Layout<'_>,
    cursor: iced::advanced::mouse::Cursor,
    viewport: &iced::Rectangle,
  ) {
    let state = tree.state.downcast_ref::<State>();
    let translation = state.translation;

    let bounds = layout.bounds();
    let Some(visible_bounds) = bounds.intersection(viewport) else {
      return;
    };

    let content_layout = layout.children().next().unwrap();

    let cursor = match cursor.position_over(bounds) {
      Some(cursor_position) => mouse::Cursor::Available(cursor_position + translation),
      _ => mouse::Cursor::Unavailable,
    };

    renderer.with_layer(visible_bounds, |renderer| {
      renderer.with_translation(
        Vector {
          x: -translation.x,
          y: -translation.y,
        },
        |renderer| {
          self.inner.draw(
            tree,
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            &Rectangle {
              x: bounds.x + translation.x,
              y: bounds.y + translation.y,
              ..bounds
            },
          );
        },
      )
    })
  }

  fn overlay<'b>(
    &'b mut self,
    tree: &'b mut Tree,
    layout: Layout,
    renderer: &Renderer,
    translation: Vector,
  ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
    let state = tree.state.downcast_ref::<State>();

    self.inner.overlay(
      tree,
      layout.children().next().unwrap(),
      renderer,
      translation - state.translation,
    )
  }
}

impl<'a, Message, Theme, Renderer> From<ScrolledColumn<'a, Message, Theme, Renderer>>
  for Element<'a, Message, Theme, Renderer>
where
  Message: 'a,
  Theme: 'a,
  Renderer: iced::advanced::Renderer + 'a,
{
  fn from(scrolled_column: ScrolledColumn<'a, Message, Theme, Renderer>) -> Self {
    Self::new(scrolled_column)
  }
}

struct State {
  translation: Vector,
}

impl State {
  fn new() -> Self {
    Self {
      translation: Vector { x: 0.0, y: 0.0 },
    }
  }

  fn clamp_y(&mut self, min: f32, max: f32) {
    self.translation.y = self.translation.y.clamp(min, max);
  }

  fn clamp_scroll(&mut self, viewport: Rectangle, content_layout: Layout) {
    self.clamp_y(0.0, content_layout.bounds().height - viewport.height);
  }

  fn scroll_into_view(
    &mut self,
    viewport: Rectangle,
    content_layout: Layout,
    child_bounds: Rectangle,
  ) {
    let top = child_bounds.y - viewport.y;
    let bottom = top - viewport.height + child_bounds.height;

    if top > bottom {
      self.clamp_y(bottom, top);
    } else {
      self.clamp_y(top, bottom);
    }

    self.clamp_scroll(viewport, content_layout);
  }

  fn scroll_child_into_view(&mut self, viewport: Rectangle, content_layout: Layout, child: usize) {
    if let Some(child_layout) = content_layout.children().nth(child) {
      self.scroll_into_view(viewport, content_layout, child_layout.bounds());
    } else {
      self.clamp_scroll(viewport, content_layout);
    }
  }
}

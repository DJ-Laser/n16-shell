use iced::widget::text::{Catalog, Style, StyleFn};

use crate::Base16Theme;

impl Catalog for Base16Theme {
  type Class<'a> = StyleFn<'a, Self>;

  fn default<'a>() -> Self::Class<'a> {
    Box::new(|_theme| Style::default())
  }

  fn style(&self, class: &Self::Class<'_>) -> Style {
    class(self)
  }
}

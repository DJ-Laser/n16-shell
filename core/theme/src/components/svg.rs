use iced::widget::svg::{Catalog, Status, Style, StyleFn};

use crate::Base16Theme;

impl Catalog for Base16Theme {
  type Class<'a> = StyleFn<'a, Self>;

  fn default<'a>() -> Self::Class<'a> {
    Box::new(|_theme, _status| Style::default())
  }

  fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
    class(self, status)
  }
}

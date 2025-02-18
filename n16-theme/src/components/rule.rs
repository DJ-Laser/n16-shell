use iced::widget::rule::{Catalog, FillMode, Style, StyleFn};

use crate::Base16Theme;

impl Catalog for Base16Theme {
  type Class<'a> = StyleFn<'a, Self>;

  fn default<'a>() -> Self::Class<'a> {
    Box::new(|theme| Style {
      color: theme.base03,
      width: 1,
      radius: 0.0.into(),
      fill_mode: FillMode::Full,
    })
  }

  fn style(&self, class: &Self::Class<'_>) -> Style {
    class(self)
  }
}

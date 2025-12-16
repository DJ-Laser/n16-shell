use iced::{
  Color,
  widget::rule::{Catalog, FillMode, Style, StyleFn},
};

use crate::Base16Theme;

impl Catalog for Base16Theme {
  type Class<'a> = StyleFn<'a, Self>;

  fn default<'a>() -> Self::Class<'a> {
    Box::new(base)
  }

  fn style(&self, class: &Self::Class<'_>) -> Style {
    class(self)
  }
}

pub fn colored(color: Color) -> Style {
  Style {
    color,
    radius: 0.0.into(),
    fill_mode: FillMode::Full,
    snap: true,
  }
}

pub fn base(theme: &Base16Theme) -> Style {
  colored(theme.base03)
}

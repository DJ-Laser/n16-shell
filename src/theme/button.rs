use iced::border;
use iced::widget::button::{Catalog, Status, Style, StyleFn};

use super::Base16Theme;

impl Catalog for Base16Theme {
  type Class<'a> = StyleFn<'a, Self>;

  fn default<'a>() -> Self::Class<'a> {
    Box::new(|theme, status| {
      let base = base(theme);
      match status {
        Status::Active | Status::Pressed => base,
        Status::Hovered => Style {
          background: Some(theme.base02.into()),
          ..base
        },
        Status::Disabled => disabled(base),
      }
    })
  }

  fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
    class(self, status)
  }
}

pub fn base(theme: &Base16Theme) -> Style {
  Style {
    background: Some(theme.base01.into()),
    text_color: theme.base05,
    border: border::rounded(2),
    ..Style::default()
  }
}

pub fn disabled(style: Style) -> Style {
  Style {
    background: style
      .background
      .map(|background| background.scale_alpha(0.5)),
    text_color: style.text_color.scale_alpha(0.5),
    ..style
  }
}

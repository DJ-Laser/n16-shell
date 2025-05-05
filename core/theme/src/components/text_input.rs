use iced::widget::text_input::{Catalog, Status, Style, StyleFn};
use iced::Border;

use crate::Base16Theme;

impl Catalog for Base16Theme {
  type Class<'a> = StyleFn<'a, Self>;

  fn default<'a>() -> Self::Class<'a> {
    Box::new(|theme, status: Status| base(theme, status))
  }

  fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
    class(self, status)
  }
}

pub fn base(theme: &Base16Theme, status: Status) -> Style {
  let active = Style {
    background: theme.base00.into(),
    border: Border {
      radius: 2.0.into(),
      width: 0.0,
      color: theme.base00,
    },
    icon: theme.base06,
    placeholder: theme.base03,
    value: theme.base05,
    selection: theme.base02,
  };

  match status {
    Status::Active => active,
    Status::Hovered => Style {
      border: Border { ..active.border },
      ..active
    },
    Status::Focused => Style {
      border: Border { ..active.border },
      ..active
    },
    Status::Disabled => Style {
      background: theme.base01.into(),
      value: active.placeholder,
      ..active
    },
  }
}

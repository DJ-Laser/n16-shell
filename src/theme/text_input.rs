use iced::{
  widget::text_input::{Catalog, Status, Style, StyleFn},
  Background, Border,
};

use super::Base16Theme;

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
    background: Background::Color(theme.base00),
    border: Border {
      radius: 2.0.into(),
      width: 1.0,
      color: theme.base0C,
    },
    icon: theme.base06,
    placeholder: theme.base03,
    value: theme.base05,
    selection: theme.base02,
  };

  match status {
    Status::Active => active,
    Status::Hovered => Style {
      border: Border {
        //color: palette.background.base.text,
        ..active.border
      },
      ..active
    },
    Status::Focused => Style {
      border: Border {
        //color: palette.primary.strong.color,
        ..active.border
      },
      ..active
    },
    Status::Disabled => Style {
      background: Background::Color(theme.base01),
      value: active.placeholder,
      ..active
    },
  }
}

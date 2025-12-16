use iced::widget::scrollable::{AutoScroll, Catalog, Rail, Scroller, Status, Style, StyleFn};
use iced::{Background, Color, Shadow, Vector, border};

use crate::Base16Theme;

impl Catalog for Base16Theme {
  type Class<'a> = StyleFn<'a, Self>;

  fn default<'a>() -> Self::Class<'a> {
    Box::new(|theme, _status| {
      let scrollbar = Rail {
        background: None,
        border: border::rounded(2),
        scroller: Scroller {
          background: Background::Color(Color::default()),
          border: border::rounded(2),
        },
      };

      let auto_scroll = AutoScroll {
        background: theme.base00.scale_alpha(0.9).into(),
        border: border::rounded(u32::MAX)
          .width(1)
          .color(theme.base05.scale_alpha(0.8)),
        shadow: Shadow {
          color: Color::BLACK.scale_alpha(0.7),
          offset: Vector::ZERO,
          blur_radius: 2.0,
        },
        icon: theme.base05.scale_alpha(0.8),
      };

      Style {
        container: Default::default(),
        vertical_rail: scrollbar,
        horizontal_rail: scrollbar,
        gap: Default::default(),
        auto_scroll,
      }
    })
  }
  fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
    class(self, status)
  }
}

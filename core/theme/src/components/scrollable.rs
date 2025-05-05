use iced::border;
use iced::widget::scrollable::{Catalog, Rail, Scroller, Status, Style, StyleFn};

use crate::Base16Theme;

impl Catalog for Base16Theme {
  type Class<'a> = StyleFn<'a, Self>;

  fn default<'a>() -> Self::Class<'a> {
    Box::new(|_theme, _status| {
      let scrollbar = Rail {
        background: None,
        border: border::rounded(2),
        scroller: Scroller {
          color: Default::default(),
          border: border::rounded(2),
        },
      };

      Style {
        container: Default::default(),
        vertical_rail: scrollbar,
        horizontal_rail: scrollbar,
        gap: Default::default(),
      }
    })
  }
  fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
    class(self, status)
  }
}

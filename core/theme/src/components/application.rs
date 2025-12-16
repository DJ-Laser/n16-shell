use iced::theme::{Base, Style};

use crate::Base16Theme;

impl Base for Base16Theme {
  fn default(_preference: iced::theme::Mode) -> Self {
    crate::DEFAULT_THEME
  }

  fn mode(&self) -> iced::theme::Mode {
    iced::theme::Mode::None
  }

  fn base(&self) -> Style {
    Style {
      background_color: self.base00,
      text_color: self.base05,
    }
  }

  fn palette(&self) -> Option<iced::theme::Palette> {
    None
  }

  fn name(&self) -> &str {
    "Base16 Theme"
  }
}

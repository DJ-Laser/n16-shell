use iced_layershell::{Appearance, DefaultStyle};

use super::Base16Theme;

impl DefaultStyle for Base16Theme {
  fn default_style(&self) -> Appearance {
    Appearance {
      background_color: self.base00,
      text_color: self.base05,
    }
  }
}

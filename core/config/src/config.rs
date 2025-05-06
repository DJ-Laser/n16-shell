use n16_theme::Base16Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
  theme: Base16Theme,
}

impl Config {
  pub fn theme(&self) -> &Base16Theme {
    &self.theme
  }
}

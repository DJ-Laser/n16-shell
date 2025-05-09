use knuffel::Decode;
use n16_theme::Base16Theme;

#[derive(Debug, Clone, Default, Decode)]
pub struct Config {
  theme: Base16Theme,
}

impl Config {
  pub fn theme(&self) -> &Base16Theme {
    &self.theme
  }
}

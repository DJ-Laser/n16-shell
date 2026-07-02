use crate::theme::Base16Theme;

#[derive(Debug, Clone, Default, knus::Decode)]
pub struct Config {
  #[knus(child)]
  theme: Base16Theme,
}

impl Config {
  pub fn theme(&self) -> &Base16Theme {
    &self.theme
  }
}

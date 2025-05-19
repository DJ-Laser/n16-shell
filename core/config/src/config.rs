use n16_theme::Base16Theme;

#[derive(Debug, Clone, Default, knuffel::Decode)]
pub struct Config {
  #[knuffel(child)]
  theme: Base16Theme,
}

impl Config {
  pub fn theme(&self) -> &Base16Theme {
    &self.theme
  }
}

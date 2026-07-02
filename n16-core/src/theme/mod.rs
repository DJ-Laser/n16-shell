use iced::{Color, color};

pub use components::*;

mod components;
mod serialize;

#[derive(Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct Base16Theme {
  pub base00: Color,
  pub base01: Color,
  pub base02: Color,
  pub base03: Color,
  pub base04: Color,
  pub base05: Color,
  pub base06: Color,
  pub base07: Color,
  pub base08: Color,
  pub base09: Color,
  pub base0A: Color,
  pub base0B: Color,
  pub base0C: Color,
  pub base0D: Color,
  pub base0E: Color,
  pub base0F: Color,
}

pub const DEFAULT_THEME: Base16Theme = Base16Theme {
  base00: color!(0x1d1f21),
  base01: color!(0x282a2e),
  base02: color!(0x373b41),
  base03: color!(0x969896),
  base04: color!(0xb4b7b4),
  base05: color!(0xc5c8c6),
  base06: color!(0xe0e0e0),
  base07: color!(0xffffff),
  base08: color!(0xcc6666),
  base09: color!(0xde935f),
  base0A: color!(0xf0c674),
  base0B: color!(0xb5bd68),
  base0C: color!(0x8abeb7),
  base0D: color!(0x81a2be),
  base0E: color!(0xb294bb),
  base0F: color!(0xa3685a),
};

impl Default for Base16Theme {
  fn default() -> Self {
    DEFAULT_THEME
  }
}

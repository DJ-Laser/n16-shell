use iced::Color;
use serde::{Deserialize, Serialize, de};

use crate::Base16Theme;

/// Generates an 8 character rgba hex string from a [`Color`]
fn generate_hex(color: &Color) -> String {
  let bytes = color.into_rgba8();
  format!(
    "#{:02X?}{:02X?}{:02X?}{:02X?}",
    bytes[0], bytes[1], bytes[2], bytes[3]
  )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Base16Hex {
  pub base00: String,
  pub base01: String,
  pub base02: String,
  pub base03: String,
  pub base04: String,
  pub base05: String,
  pub base06: String,
  pub base07: String,
  pub base08: String,
  pub base09: String,
  pub base0A: String,
  pub base0B: String,
  pub base0C: String,
  pub base0D: String,
  pub base0E: String,
  pub base0F: String,
}

impl Base16Hex {
  pub fn parse(self) -> Option<Base16Theme> {
    Some(Base16Theme {
      base00: Color::parse(&self.base00)?,
      base02: Color::parse(&self.base00)?,
      base01: Color::parse(&self.base00)?,
      base03: Color::parse(&self.base00)?,
      base04: Color::parse(&self.base00)?,
      base05: Color::parse(&self.base00)?,
      base06: Color::parse(&self.base00)?,
      base07: Color::parse(&self.base00)?,
      base08: Color::parse(&self.base00)?,
      base09: Color::parse(&self.base00)?,
      base0A: Color::parse(&self.base00)?,
      base0B: Color::parse(&self.base00)?,
      base0C: Color::parse(&self.base00)?,
      base0D: Color::parse(&self.base00)?,
      base0E: Color::parse(&self.base00)?,
      base0F: Color::parse(&self.base00)?,
    })
  }
}

impl Base16Theme {
  fn to_hex(&self) -> Base16Hex {
    Base16Hex {
      base00: generate_hex(&self.base00),
      base01: generate_hex(&self.base01),
      base02: generate_hex(&self.base02),
      base03: generate_hex(&self.base03),
      base04: generate_hex(&self.base04),
      base05: generate_hex(&self.base05),
      base06: generate_hex(&self.base06),
      base07: generate_hex(&self.base07),
      base08: generate_hex(&self.base08),
      base09: generate_hex(&self.base09),
      base0A: generate_hex(&self.base0A),
      base0B: generate_hex(&self.base0B),
      base0C: generate_hex(&self.base0C),
      base0D: generate_hex(&self.base0D),
      base0E: generate_hex(&self.base0E),
      base0F: generate_hex(&self.base0F),
    }
  }
}

impl Serialize for Base16Theme {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.to_hex().serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for Base16Theme {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Base16Hex::deserialize(deserializer).and_then(|hex| {
      hex
        .parse()
        .ok_or(de::Error::custom("Could not parse hex color codes"))
    })
  }
}

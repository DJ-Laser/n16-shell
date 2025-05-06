use iced::Color;

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

impl Default for Base16Theme {
  fn default() -> Self {
    Base16Theme {
      base00: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base01: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base02: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base03: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base04: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base05: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base06: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base07: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base08: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base09: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base0A: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base0B: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base0C: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base0D: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base0E: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
      base0F: Color {
        r: 0.11372549,
        g: 0.12156863,
        b: 0.12941177,
        a: 1.0,
      },
    }
  }
}

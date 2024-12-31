use iced::Color;

pub mod application;
pub mod button;
pub mod container;
pub mod scrollable;
pub mod text;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Base16Hex<'a> {
  pub base00: &'a str,
  pub base01: &'a str,
  pub base02: &'a str,
  pub base03: &'a str,
  pub base04: &'a str,
  pub base05: &'a str,
  pub base06: &'a str,
  pub base07: &'a str,
  pub base08: &'a str,
  pub base09: &'a str,
  pub base0A: &'a str,
  pub base0B: &'a str,
  pub base0C: &'a str,
  pub base0D: &'a str,
  pub base0E: &'a str,
  pub base0F: &'a str,
}

impl<'a> Base16Hex<'a> {
  pub fn parse(self) -> Option<Base16Theme> {
    println!("{:?}", self.base0B);
    println!("{:?}", Color::parse(self.base0B));
    Some(Base16Theme {
      base00: Color::parse(self.base00)?,
      base02: Color::parse(self.base00)?,
      base01: Color::parse(self.base00)?,
      base03: Color::parse(self.base00)?,
      base04: Color::parse(self.base00)?,
      base05: Color::parse(self.base00)?,
      base06: Color::parse(self.base00)?,
      base07: Color::parse(self.base00)?,
      base08: Color::parse(self.base00)?,
      base09: Color::parse(self.base00)?,
      base0A: Color::parse(self.base00)?,
      base0B: Color::parse(self.base00)?,
      base0C: Color::parse(self.base00)?,
      base0D: Color::parse(self.base00)?,
      base0E: Color::parse(self.base00)?,
      base0F: Color::parse(self.base00)?,
    })
  }
}

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

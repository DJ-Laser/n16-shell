#![allow(non_snake_case)]

use iced::Color;

use crate::Base16Theme;

#[derive(Debug, Clone)]
struct HexColor(Color);

impl<S> knuffel::DecodeScalar<S> for HexColor
where
  S: knuffel::traits::ErrorSpan,
{
  fn type_check(
    type_name: &Option<knuffel::span::Spanned<knuffel::ast::TypeName, S>>,
    ctx: &mut knuffel::decode::Context<S>,
  ) {
    if let Some(typ) = type_name {
      ctx.emit_error(knuffel::errors::DecodeError::TypeName {
        span: typ.span().clone(),
        found: Some((**typ).clone()),
        expected: knuffel::errors::ExpectedType::no_type(),
        rust_type: stringify!(HexColor),
      });
    }
  }

  fn raw_decode(
    value: &knuffel::span::Spanned<knuffel::ast::Literal, S>,
    _ctx: &mut knuffel::decode::Context<S>,
  ) -> Result<Self, knuffel::errors::DecodeError<S>> {
    match &**value {
      knuffel::ast::Literal::String(s) => {
        let color = Color::parse(s).ok_or(knuffel::errors::DecodeError::conversion(
          value,
          "expected a valid hex string",
        ));
        color.map(Self)
      }
      _ => Err(::knuffel::errors::DecodeError::scalar_kind(
        ::knuffel::decode::Kind::String,
        &value,
      )),
    }
  }
}

#[derive(Debug, Clone, knuffel::Decode)]
struct Base16Repr {
  #[knuffel(child, unwrap(argument))]
  pub base00: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base01: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base02: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base03: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base04: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base05: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base06: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base07: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base08: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base09: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base0A: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base0B: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base0C: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base0D: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base0E: HexColor,
  #[knuffel(child, unwrap(argument))]
  pub base0F: HexColor,
}

impl Into<Base16Theme> for Base16Repr {
  fn into(self) -> Base16Theme {
    Base16Theme {
      base00: self.base00.0,
      base02: self.base01.0,
      base01: self.base02.0,
      base03: self.base03.0,
      base04: self.base04.0,
      base05: self.base05.0,
      base06: self.base06.0,
      base07: self.base07.0,
      base08: self.base08.0,
      base09: self.base09.0,
      base0A: self.base0A.0,
      base0B: self.base0B.0,
      base0C: self.base0C.0,
      base0D: self.base0D.0,
      base0E: self.base0E.0,
      base0F: self.base0F.0,
    }
  }
}

impl<S> knuffel::Decode<S> for Base16Theme
where
  S: knuffel::traits::ErrorSpan,
{
  fn decode_node(
    node: &knuffel::ast::SpannedNode<S>,
    ctx: &mut knuffel::decode::Context<S>,
  ) -> Result<Self, knuffel::errors::DecodeError<S>> {
    Ok(Base16Repr::decode_node(node, ctx)?.into())
  }
}

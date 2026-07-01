use std::str::FromStr;

use iced::Color;

use crate::Base16Theme;

#[derive(Debug, Clone)]
struct HexColor(Color);

impl<S> knus::DecodeScalar<S> for HexColor
where
  S: knus::traits::ErrorSpan,
{
  fn type_check(
    type_name: &Option<knus::span::Spanned<knus::ast::TypeName, S>>,
    ctx: &mut knus::decode::Context<S>,
  ) {
    if let Some(typ) = type_name {
      ctx.emit_error(knus::errors::DecodeError::TypeName {
        span: typ.span().clone(),
        found: Some((**typ).clone()),
        expected: knus::errors::ExpectedType::no_type(),
        rust_type: stringify!(HexColor),
      });
    }
  }

  fn raw_decode(
    value: &knus::span::Spanned<knus::ast::Literal, S>,
    _ctx: &mut knus::decode::Context<S>,
  ) -> Result<Self, knus::errors::DecodeError<S>> {
    match &**value {
      knus::ast::Literal::String(s) => {
        let color = Color::from_str(s)
          .map_err(|_| knus::errors::DecodeError::conversion(value, "expected a valid hex string"));
        color.map(Self)
      }
      _ => Err(::knus::errors::DecodeError::scalar_kind(
        ::knus::decode::Kind::String,
        value,
      )),
    }
  }
}

#[derive(Debug, Clone, knus::Decode)]
struct Base16Repr {
  #[knus(child, unwrap(argument))]
  pub base00: HexColor,
  #[knus(child, unwrap(argument))]
  pub base01: HexColor,
  #[knus(child, unwrap(argument))]
  pub base02: HexColor,
  #[knus(child, unwrap(argument))]
  pub base03: HexColor,
  #[knus(child, unwrap(argument))]
  pub base04: HexColor,
  #[knus(child, unwrap(argument))]
  pub base05: HexColor,
  #[knus(child, unwrap(argument))]
  pub base06: HexColor,
  #[knus(child, unwrap(argument))]
  pub base07: HexColor,
  #[knus(child, unwrap(argument))]
  pub base08: HexColor,
  #[knus(child, unwrap(argument))]
  pub base09: HexColor,
  #[knus(child, unwrap(argument))]
  pub base0a: HexColor,
  #[knus(child, unwrap(argument))]
  pub base0b: HexColor,
  #[knus(child, unwrap(argument))]
  pub base0c: HexColor,
  #[knus(child, unwrap(argument))]
  pub base0d: HexColor,
  #[knus(child, unwrap(argument))]
  pub base0e: HexColor,
  #[knus(child, unwrap(argument))]
  pub base0f: HexColor,
}

impl From<Base16Repr> for Base16Theme {
  fn from(val: Base16Repr) -> Self {
    Base16Theme {
      base00: val.base00.0,
      base02: val.base01.0,
      base01: val.base02.0,
      base03: val.base03.0,
      base04: val.base04.0,
      base05: val.base05.0,
      base06: val.base06.0,
      base07: val.base07.0,
      base08: val.base08.0,
      base09: val.base09.0,
      base0A: val.base0a.0,
      base0B: val.base0b.0,
      base0C: val.base0c.0,
      base0D: val.base0d.0,
      base0E: val.base0e.0,
      base0F: val.base0f.0,
    }
  }
}

impl<S> knus::Decode<S> for Base16Theme
where
  S: knus::traits::ErrorSpan,
{
  fn decode_node(
    node: &knus::ast::SpannedNode<S>,
    ctx: &mut knus::decode::Context<S>,
  ) -> Result<Self, knus::errors::DecodeError<S>> {
    Ok(Base16Repr::decode_node(node, ctx)?.into())
  }
}

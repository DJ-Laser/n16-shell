use iced_layershell::actions::LayershellCustomActionsWithId;

#[derive(Debug, Clone)]
pub enum Message {
  Init,
  Launcher(n16_launcher::Message),
  LayershellAction(LayershellCustomActionsWithId),
}

impl From<LayershellCustomActionsWithId> for Message {
  fn from(value: LayershellCustomActionsWithId) -> Self {
    Self::LayershellAction(value)
  }
}

impl TryInto<LayershellCustomActionsWithId> for Message {
  type Error = Self;

  fn try_into(self) -> Result<LayershellCustomActionsWithId, Self::Error> {
    match self {
      Self::LayershellAction(action) => Ok(action),
      _ => Err(self),
    }
  }
}

use iced::futures::channel::oneshot;
use iced_layershell::actions::LayershellCustomActionsWithId;
use n16_ipc::Reply;

#[derive(Debug)]
pub enum Message {
  Launcher(n16_launcher::Message),
  Bar(n16_bar::Message),
  LayershellAction(LayershellCustomActionsWithId),
  Request(n16_ipc::Request, oneshot::Sender<Reply>),
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

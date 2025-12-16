use iced::futures::channel::oneshot;
use iced_layershell::actions::LayershellCustomActionWithId;
use n16_ipc::Reply;

#[derive(Debug)]
pub enum Message {
  Launcher(n16_launcher::Message),
  Bar(n16_bar::Message),
  LayershellAction(LayershellCustomActionWithId),
  Request(n16_ipc::Request, oneshot::Sender<Reply>),
  WindowClose(iced::window::Id),
}

impl From<LayershellCustomActionWithId> for Message {
  fn from(value: LayershellCustomActionWithId) -> Self {
    Self::LayershellAction(value)
  }
}

impl TryInto<LayershellCustomActionWithId> for Message {
  type Error = Self;

  fn try_into(self) -> Result<LayershellCustomActionWithId, Self::Error> {
    match self {
      Self::LayershellAction(action) => Ok(action),
      _ => Err(self),
    }
  }
}

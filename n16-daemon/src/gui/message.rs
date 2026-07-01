use iced::futures::channel::oneshot;
use iced_layershell::actions::LayerShellCustomActionWithId;
use n16_ipc::Reply;

#[derive(Debug)]
pub enum Message {
  Launcher(n16_launcher::Message),
  Bar(n16_bar::Message),
  LayershellAction(LayerShellCustomActionWithId),
  Request(n16_ipc::Request, oneshot::Sender<Reply>),
  WindowClose(iced::window::Id),
}

impl From<LayerShellCustomActionWithId> for Message {
  fn from(value: LayerShellCustomActionWithId) -> Self {
    Self::LayershellAction(value)
  }
}

impl TryInto<LayerShellCustomActionWithId> for Message {
  type Error = Self;

  fn try_into(self) -> Result<LayerShellCustomActionWithId, Self::Error> {
    match self {
      Self::LayershellAction(action) => Ok(action),
      _ => Err(self),
    }
  }
}

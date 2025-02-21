use iced::Task;
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, NewLayerShellSettings};

use n16_application::{
  ipc::RequestHandler,
  single_window::{ShellAction, ShellApplication},
};
use n16_ipc::bar::{self, Request, Response};

#[derive(Debug, Clone)]
pub enum Message {
  Hide,
  Show,
}

impl TryInto<ShellAction> for Message {
  type Error = Self;

  fn try_into(self) -> Result<ShellAction, Self::Error> {
    match self {
      Self::Show => Ok(ShellAction::Open(NewLayerShellSettings {
        size: Some((0, 50)),
        anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
        keyboard_interactivity: KeyboardInteractivity::None,
        exclusive_zone: Some(50),
        ..Default::default()
      })),

      Self::Hide => Ok(ShellAction::Close),
    }
  }
}

pub struct Bar {}

impl Bar {
  pub fn new() -> Self {
    Self {}
  }
}

impl ShellApplication for Bar {
  type Message = Message;

  fn update(&mut self, message: Self::Message) -> iced::Task<Self::Message> {
    match message {
      _ => Task::none(),
    }
  }

  fn view(&self) -> iced::Element<'_, Self::Message, n16_theme::Base16Theme> {
    "todo".into()
  }
}

impl RequestHandler for Bar {
  type Request = bar::Request;

  type Message = Message;

  fn handle_request(
    &mut self,
    request: Self::Request,
    reply_channel: iced::futures::channel::oneshot::Sender<n16_ipc::Reply>,
  ) -> Task<Self::Message> {
    match request {
      Request::Show => {
        let _ = reply_channel.send(Response::handled().reply_ok());
        Task::done(Message::Show)
      }

      Request::Hide => {
        let _ = reply_channel.send(Response::handled().reply_ok());
        Task::done(Message::Hide)
      }
    }
  }
}

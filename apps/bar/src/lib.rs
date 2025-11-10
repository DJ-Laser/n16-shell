use component::clock;
use iced::widget::{Space, row};
use iced::{Length, Subscription, Task, time};
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, NewLayerShellSettings};

use n16_application::{
  ipc::RequestHandler,
  single_window::{ShellAction, ShellApplication},
};
use n16_ipc::bar::{self, Request, Response};

mod component;

#[derive(Debug, Clone)]
pub enum Message {
  Tick(chrono::DateTime<chrono::Local>),
  Hide,
  Show,
}

impl TryInto<ShellAction> for Message {
  type Error = Self;

  fn try_into(self) -> Result<ShellAction, Self::Error> {
    match self {
      Self::Show => Ok(ShellAction::Open(NewLayerShellSettings {
        size: Some((0, 30)),
        anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
        keyboard_interactivity: KeyboardInteractivity::None,
        exclusive_zone: Some(30),
        ..Default::default()
      })),

      Self::Hide => Ok(ShellAction::Close),

      _ => Err(self),
    }
  }
}

pub struct Bar {
  now: chrono::DateTime<chrono::Local>,
}

impl Bar {
  pub fn new() -> Self {
    Self::default()
  }
}

impl ShellApplication for Bar {
  type Message = Message;

  fn update(&mut self, message: Self::Message) -> iced::Task<Self::Message> {
    match message {
      Message::Tick(time) => {
        self.now = time;
        Task::none()
      }

      _ => Task::none(),
    }
  }

  fn view(&self) -> iced::Element<'_, Self::Message, n16_theme::Base16Theme> {
    row![
      Space::with_width(Length::Fill),
      clock::view(self.now).into()
    ]
    .padding(5)
    .into()
  }

  fn subscription(&self) -> Subscription<Message> {
    time::every(time::Duration::from_millis(500))
      .map(|_| Message::Tick(chrono::offset::Local::now()))
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

impl Default for Bar {
  fn default() -> Self {
    Self {
      now: chrono::offset::Local::now(),
    }
  }
}

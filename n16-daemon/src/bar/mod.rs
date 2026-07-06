use iced::{
  Length, Subscription, Task, time,
  widget::{Space, row},
  window,
};
use iced_layershell::{
  reexport::{Anchor, KeyboardInteractivity, NewLayerShellSettings},
  settings::{LayerShellSettings, StartMode},
  to_layer_message,
};
use n16_core::theme::{self};
use n16_ipc::{Response, bar::Request};

use crate::{
  application::{ApplicationRequest, N16Application, RequestChannel},
  bar::components::clock,
};

mod components;

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
  Tick(chrono::DateTime<chrono::Local>),
  ShowBar(bool),
  RequestRecieved(ApplicationRequest<Request>),
}

pub struct Bar {
  now: chrono::DateTime<chrono::Local>,
  window_id: Option<window::Id>,
}

impl Bar {
  fn handle_request(&mut self, request: ApplicationRequest<Request>) -> Task<Message> {
    match request.kind() {
      Request::Show => {
        request.reply(Response::Handled);
        Task::done(Message::ShowBar(true))
      }
      Request::Hide => {
        request.reply(Response::Handled);
        Task::done(Message::ShowBar(false))
      }
    }
  }

  fn show_bar(&mut self) -> Task<Message> {
    if self.window_id.is_some() {
      return Task::none();
    }

    let (id, task) = Message::layershell_open(NewLayerShellSettings {
      size: Some((0, 30)),
      anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
      keyboard_interactivity: KeyboardInteractivity::None,
      exclusive_zone: Some(30),
      ..Default::default()
    });

    self.window_id = Some(id);
    task
  }

  fn hide_bar(&mut self) -> Task<Message> {
    let Some(window_id) = self.window_id else {
      return Task::none();
    };

    self.window_id = None;
    window::close(window_id)
  }
}

impl Bar {
  pub fn new() -> Self {
    Self {
      now: chrono::offset::Local::now(),
      window_id: None,
    }
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Tick(time) => {
        self.now = time;
        Task::none()
      }

      Message::ShowBar(show) => {
        if show {
          self.show_bar()
        } else {
          self.hide_bar()
        }
      }

      Message::RequestRecieved(request) => self.handle_request(request),

      _ => unreachable!(),
    }
  }

  pub fn view(&self, _id: window::Id) -> iced::Element<'_, Message, theme::Base16Theme> {
    row![
      Space::new().width(Length::Fill),
      clock::view(self.now).into()
    ]
    .padding(5)
    .into()
  }

  pub fn subscription(&self) -> Subscription<Message> {
    time::every(time::Duration::from_millis(500))
      .map(|_| Message::Tick(chrono::offset::Local::now()))
  }
}

impl N16Application for Bar {
  type Request = Request;

  fn run(request_rx: RequestChannel<Self::Request>) {
    let _ = iced_layershell::daemon(
      move || {
        (
          Bar::new(),
          Task::stream(request_rx.clone()).map(Message::RequestRecieved),
        )
      },
      "n16_bar",
      Bar::update,
      Bar::view,
    )
    .subscription(Bar::subscription)
    .layer_settings(LayerShellSettings {
      start_mode: StartMode::Background,
      ..Default::default()
    })
    .run();
  }
}

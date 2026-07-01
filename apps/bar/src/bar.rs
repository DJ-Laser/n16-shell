use iced::{
  Length, Subscription, Task, time,
  widget::{Space, row},
  window,
};
use iced_layershell::{
  reexport::{Anchor, KeyboardInteractivity, NewLayerShellSettings},
  to_layer_message,
};
use n16_theme::Base16Theme;

pub mod clock;

type Component = iced::Element<'static, Message, Base16Theme, iced::Renderer>;

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
  Tick(chrono::DateTime<chrono::Local>),
  ShowBar(bool),
}

pub struct Bar {
  now: chrono::DateTime<chrono::Local>,
  window_id: Option<window::Id>,
}

impl Bar {
  pub fn new() -> Self {
    Self {
      now: chrono::offset::Local::now(),
      window_id: None,
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

      _ => unreachable!(),
    }
  }

  pub fn view(&self, _id: window::Id) -> iced::Element<'_, Message, n16_theme::Base16Theme> {
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

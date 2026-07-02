use iced::{Element, Subscription, Task, window};
use iced_layershell::{
  reexport::{Anchor, NewLayerShellSettings},
  settings::{LayerShellSettings, StartMode},
  to_layer_message,
};
use listings::{Listing, Provider};
use n16_core::{
  application::{ApplicationRequest, N16Application, RequestChannel},
  theme::Base16Theme,
};
use n16_ipc::{Response, launcher::Request};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
  launcher::Launcher,
  providers::{ApplicationProvider, PowerManagementProvider},
};

mod calculator;
mod component;
mod launcher;
pub mod listings;
pub mod providers;

type Providers = Arc<Mutex<Vec<Box<dyn Provider>>>>;
type Listings = Vec<Box<dyn Listing>>;

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
enum Message {
  RequestRecieved(ApplicationRequest<Request>),
  Launcher(launcher::Message),
  Close,
}

pub struct LauncherDaemon {
  providers: Providers,
  launcher_window: Option<(window::Id, Launcher)>,
}

impl LauncherDaemon {
  #[expect(clippy::new_without_default)]
  pub fn new() -> Self {
    Self {
      providers: Self::setup_providers(),
      launcher_window: None,
    }
  }

  pub fn setup_providers() -> Providers {
    let providers: Vec<Box<dyn Provider>> = vec![
      Box::new(ApplicationProvider::new()),
      Box::new(PowerManagementProvider::new()),
    ];

    Arc::new(Mutex::new(providers))
  }

  fn open_launcher(&mut self) -> Task<Message> {
    if self.launcher_window.is_some() {
      return Task::none();
    }

    let (id, window_task) = Message::layershell_open(NewLayerShellSettings {
      size: Some((1000, 600)),
      anchor: Anchor::Top,
      margin: Some((200, 0, 0, 0)),
      ..Default::default()
    });

    let (launcher_window, launcher_task) = Launcher::new(self.providers.clone());
    self.launcher_window = Some((id, launcher_window));

    Task::batch([
      window_task.chain(Task::done(Message::Launcher(launcher::Message::FocusInput))),
      launcher_task.map(Message::Launcher),
    ])
  }

  fn handle_request(&mut self, request: ApplicationRequest<Request>) -> Task<Message> {
    match request.kind() {
      Request::Open => {
        request.reply(Response::Handled);
        self.open_launcher()
      }
      Request::Close => {
        request.reply(Response::Handled);
        Task::done(Message::Close)
      }
    }
  }
}

impl LauncherDaemon {
  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::RequestRecieved(request) => self.handle_request(request),
      Message::Launcher(message) => {
        if let Some((_, launcher)) = self.launcher_window.as_mut() {
          match launcher.update(message) {
            launcher::Action::Task(task) => task.map(Message::Launcher),
            launcher::Action::Close => Task::done(Message::Close),
          }
        } else {
          Task::none()
        }
      }

      Message::Close => {
        if let Some((id, _)) = self.launcher_window {
          self.launcher_window = None;
          window::close(id)
        } else {
          Task::none()
        }
      }

      _ => unreachable!(),
    }
  }

  fn view(&self, window_id: window::Id) -> Element<'_, Message, Base16Theme> {
    if let Some((id, launcher)) = &self.launcher_window
      && *id == window_id
    {
      launcher.view().map(Message::Launcher)
    } else {
      "".into()
    }
  }

  fn subscription(&self) -> Subscription<Message> {
    self
      .launcher_window
      .as_ref()
      .map(|(_, launcher)| launcher.subscription().map(Message::Launcher))
      .unwrap_or(Subscription::none())
  }
}

impl N16Application for LauncherDaemon {
  type Request = Request;

  fn run(request_rx: RequestChannel<Self::Request>) {
    let _ = iced_layershell::daemon(
      move || {
        (
          LauncherDaemon::new(),
          Task::batch([Task::stream(request_rx.clone()).map(Message::RequestRecieved)]),
        )
      },
      "n16_launcher",
      LauncherDaemon::update,
      LauncherDaemon::view,
    )
    .layer_settings(LayerShellSettings {
      start_mode: StartMode::Background,
      ..Default::default()
    })
    .subscription(LauncherDaemon::subscription)
    .run();
  }
}

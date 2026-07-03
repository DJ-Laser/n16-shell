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
use std::{collections::HashMap, sync::Arc};
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
  Launcher(window::Id, launcher::Message),
  Close(window::Id),
}

pub struct LauncherDaemon {
  providers: Providers,
  launcher_windows: HashMap<window::Id, Launcher>,
}

impl LauncherDaemon {
  #[expect(clippy::new_without_default)]
  pub fn new() -> Self {
    Self {
      providers: Self::setup_providers(),
      launcher_windows: HashMap::new(),
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
    if !self.launcher_windows.is_empty() {
      return Task::none();
    }

    let (id, window_task) = Message::layershell_open(NewLayerShellSettings {
      size: Some((1000, 600)),
      anchor: Anchor::Top,
      margin: Some((200, 0, 0, 0)),
      ..Default::default()
    });

    let (launcher_window, launcher_task) = Launcher::new(self.providers.clone());
    self.launcher_windows.insert(id, launcher_window);

    Task::batch([
      window_task.chain(Task::done(Message::Launcher(
        id,
        launcher::Message::FocusInput,
      ))),
      launcher_task.map(move |m| Message::Launcher(id, m)),
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
        window::latest().and_then(|id| Task::done(Message::Close(id)))
      }
    }
  }
}

impl LauncherDaemon {
  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::RequestRecieved(request) => self.handle_request(request),
      Message::Launcher(id, message) => {
        if let Some(launcher) = self.launcher_windows.get_mut(&id) {
          match launcher.update(message) {
            launcher::Action::Task(task) => task.map(move |m| Message::Launcher(id, m)),
            launcher::Action::Close => Task::done(Message::Close(id)),
          }
        } else {
          Task::none()
        }
      }

      Message::Close(id) => {
        if self.launcher_windows.contains_key(&id) {
          self.launcher_windows.remove(&id);
          window::close(id)
        } else {
          Task::none()
        }
      }

      _ => unreachable!(),
    }
  }

  fn view(&self, window_id: window::Id) -> Element<'_, Message, Base16Theme> {
    if let Some(launcher) = &self.launcher_windows.get(&window_id) {
      launcher
        .view()
        .map(move |m| Message::Launcher(window_id, m))
    } else {
      "".into()
    }
  }

  fn subscription(&self) -> Subscription<Message> {
    Subscription::batch(self.launcher_windows.iter().map(|(id, launcher)| {
      launcher
        .subscription(*id)
        .with(*id)
        .map(|(id, m)| Message::Launcher(id, m))
    }))
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

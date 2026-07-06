use iced::{Element, Subscription, Task, window};
use iced_layershell::{
  reexport::{Anchor, NewLayerShellSettings},
  settings::{LayerShellSettings, StartMode},
  to_layer_message,
};
use n16_core::theme::Base16Theme;
use n16_ipc::{Response, launcher::Request};
use std::collections::HashMap;

use crate::{
  application::{ApplicationRequest, N16Application, RequestChannel},
  launcher::{
    gui::Launcher,
    providers::{
      ApplicationProvider, CalculatorProvider, PowerManagementProvider, Providers, ProvidersBuilder,
    },
  },
};

mod component;
mod gui;
pub mod providers;

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
enum Message {
  RequestRecieved(ApplicationRequest<Request>),
  Launcher(window::Id, gui::Message),
  Close(window::Id),
}

pub struct LauncherDaemon {
  providers: Providers,
  launcher_windows: HashMap<window::Id, Launcher>,
}

impl LauncherDaemon {
  pub fn new() -> Self {
    Self {
      providers: Self::setup_providers(),
      launcher_windows: HashMap::new(),
    }
  }

  pub fn setup_providers() -> Providers {
    let mut builder = ProvidersBuilder::new();

    builder.add_provider::<CalculatorProvider>();
    builder.add_provider::<ApplicationProvider>();
    builder.add_provider::<PowerManagementProvider>();

    builder.build()
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
      window_task,
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
            gui::Action::Task(task) => task.map(move |m| Message::Launcher(id, m)),
            gui::Action::Close => Task::done(Message::Close(id)),
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

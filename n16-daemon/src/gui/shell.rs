use std::ops::ControlFlow;

use iced::{Element, Subscription, Task, window};
use n16_ipc::{Request, Response};

use crate::ipc::run_ipc_server;

use super::Message;
use n16_application::{ipc::RequestHandler, single_window::SingleApplicationManager};
use n16_theme::Base16Theme;

pub struct Shell {
  config: n16_config::Config,

  launcher: SingleApplicationManager<n16_launcher::Launcher, Message>,
  bar: SingleApplicationManager<n16_bar::Bar, Message>,
}

impl Shell {
  pub fn new() -> Self {
    let mut launcher = n16_launcher::Launcher::new();
    launcher.add_provider(n16_launcher::providers::ApplicationProvider::new());
    launcher.add_provider(n16_launcher::providers::PowerManagementProvider::new());

    let bar = n16_bar::Bar::new();

    Self {
      config: n16_config::load_config_file().unwrap_or_default(),
      launcher: SingleApplicationManager::new(launcher, Message::Launcher),
      bar: SingleApplicationManager::new(bar, Message::Bar),
    }
  }

  pub fn theme(&self) -> Base16Theme {
    self.config.theme().clone()
  }

  fn try_view(&self, window: window::Id) -> ControlFlow<Element<'_, Message, Base16Theme>> {
    self.launcher.view(window)?;
    self.bar.view(window)?;
    ControlFlow::Continue(())
  }

  pub fn view(&self, window: window::Id) -> Element<'_, Message, Base16Theme> {
    self
      .try_view(window)
      .break_value()
      .unwrap_or_else(|| "ERROR: Unclaimed window".into())
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Launcher(launcher_message) => self.launcher.update(launcher_message),
      Message::Bar(bar_message) => self.bar.update(bar_message),

      Message::Request(request, reply_channel) => self.handle_request(request, reply_channel),

      Message::LayershellAction(_) => Task::none(),

      Message::WindowClose(window) => {
        if let ControlFlow::Continue(()) = self.remove_window(window) {
          eprintln!("Window '{window}' closed but not registered.")
        }

        Task::none()
      }
    }
  }

  fn remove_window(&mut self, window: window::Id) -> ControlFlow<()> {
    self.launcher.remove_window(window)?;
    self.bar.remove_window(window)?;

    ControlFlow::Continue(())
  }

  pub fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([
      Subscription::run(run_ipc_server)
        .map(|(request, reply_channel)| Message::Request(request, reply_channel)),
      self.launcher.subscription(),
      self.bar.subscription(),
      iced::window::close_events().map(Message::WindowClose),
    ])
  }
}

impl RequestHandler for Shell {
  type Request = Request;
  type Message = Message;

  fn handle_request(
    &mut self,
    request: Self::Request,
    reply_channel: iced::futures::channel::oneshot::Sender<n16_ipc::Reply>,
  ) -> Task<Self::Message> {
    match request {
      Request::Version => {
        let _ = reply_channel.send(Response::version().reply_ok());
        Task::none()
      }

      Request::Ping => {
        let _ = reply_channel.send(Response::Ping.reply_ok());
        Task::none()
      }

      Request::Launcher(launcher_request) => self
        .launcher
        .handle_request(launcher_request, reply_channel),

      Request::Bar(bar_request) => self.bar.handle_request(bar_request, reply_channel),
    }
  }
}

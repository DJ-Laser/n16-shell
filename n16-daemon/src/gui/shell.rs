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
  pub fn new(launcher: n16_launcher::Launcher, bar: n16_bar::Bar) -> (Self, Task<Message>) {
    (
      Self {
        config: n16_config::load_config_file().unwrap_or_default(),
        launcher: SingleApplicationManager::new(launcher, Message::Launcher),
        bar: SingleApplicationManager::new(bar, Message::Bar),
      },
      Task::done(Message::Bar(n16_bar::Message::Show)),
    )
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
    }
  }

  pub fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([
      Subscription::run(run_ipc_server)
        .map(|(request, reply_channel)| Message::Request(request, reply_channel)),
      self.launcher.subscription(),
      self.bar.subscription(),
    ])
  }

  pub fn remove_id(&mut self, window: window::Id) {
    self.launcher.remove_id(window);
    self.bar.remove_id(window);
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

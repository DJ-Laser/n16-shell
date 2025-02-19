use std::ops::ControlFlow;

use iced::{window, Element, Subscription, Task};

use crate::daemon::ipc::run_ipc_server;

use super::Message;
use n16_application::single_window::SingleApplicationManager;
use n16_launcher::Launcher;
use n16_theme::Base16Theme;

pub struct Shell {
  launcher: SingleApplicationManager<Launcher, Message>,
}

impl Shell {
  pub fn new(launcher: Launcher) -> (Self, Task<Message>) {
    (
      Self {
        launcher: SingleApplicationManager::new(launcher, Message::Launcher),
      },
      Task::done(Message::Init),
    )
  }

  fn try_view(&self, window: window::Id) -> ControlFlow<Element<'_, Message, Base16Theme>> {
    self.launcher.view(window)?;
    ControlFlow::Continue(())
  }

  pub fn view(&self, window: window::Id) -> Element<'_, Message, Base16Theme> {
    self
      .try_view(window)
      .break_value()
      .expect("Shell::view should not be called with an unclaimed window")
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Init => Task::none(),

      Message::Launcher(launcher_message) => self.launcher.update(launcher_message),

      _ => Task::none(),
    }
  }

  pub fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([
      Subscription::run(run_ipc_server)
        .map(|(request, reply_channel)| Message::Request(request, reply_channel)),
      self.launcher.subscription().map(Message::Launcher),
    ])
  }

  pub fn remove_id(&mut self, window: window::Id) {
    println!("remove_id called with window {:?}", window);
  }
}

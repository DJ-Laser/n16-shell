use std::ops::ControlFlow;

use iced::{window, Element, Subscription, Task};
use iced_layershell::actions::LayershellCustomActionsWithId;
use n16_theme::Base16Theme;

use crate::{ipc::RequestHandler, ShellMessage};

pub trait ShellApplication {
  type Message: ShellMessage + TryInto<LayershellCustomActionsWithId, Error = Self::Message>;

  fn update(&mut self, message: Self::Message) -> Task<Self::Message>;

  fn view(&self, id: window::Id) -> Element<'_, Self::Message, Base16Theme>;

  fn subscription(&self) -> Subscription<Self::Message> {
    Subscription::none()
  }
}

pub struct MultiApplicationManager<A: ShellApplication> {
  application: A,
  windows: Vec<window::Id>,
}

impl<A: ShellApplication> MultiApplicationManager<A> {
  pub fn new(application: A) -> Self {
    Self {
      application,
      windows: Vec::new(),
    }
  }

  pub fn view(&self, window: window::Id) -> ControlFlow<Element<'_, A::Message, Base16Theme>> {
    self
      .windows
      .iter()
      .find(|application_window| **application_window == window)
      .map(|_| self.application.view(window))
      .map_or(ControlFlow::Continue(()), ControlFlow::Break)
  }

  pub fn update(&mut self, message: A::Message) -> Task<A::Message> {
    self.application.update(message)
  }
}

impl<A> RequestHandler for MultiApplicationManager<A>
where
  A: ShellApplication,
  A: RequestHandler<Message = <A as ShellApplication>::Message>,
{
  type Request = A::Request;
  type Message = <A as RequestHandler>::Message;

  fn handle_request(
    &mut self,
    request: Self::Request,
    reply_channel: iced::futures::channel::oneshot::Sender<n16_ipc::Reply>,
  ) -> Task<Self::Message> {
    self.application.handle_request(request, reply_channel)
  }
}

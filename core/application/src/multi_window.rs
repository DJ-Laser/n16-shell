use std::ops::ControlFlow;

use iced::{Element, Subscription, Task, window};
use iced_layershell::actions::LayershellCustomActionWithId;
use n16_theme::Base16Theme;

use crate::{ShellMessage, ipc::RequestHandler, subscription};

pub trait ShellApplication {
  type Message: ShellMessage + TryInto<LayershellCustomActionWithId, Error = Self::Message>;

  fn update(&mut self, message: Self::Message) -> Task<Self::Message>;

  fn view(&self, id: window::Id) -> Element<'_, Self::Message, Base16Theme>;

  fn subscription(&self) -> Subscription<Self::Message> {
    Subscription::none()
  }
}

pub struct MultiApplicationManager<A: ShellApplication, M: From<LayershellCustomActionWithId>> {
  application: A,
  windows: Vec<window::Id>,
  map_fn: fn(A::Message) -> M,
}

impl<A: ShellApplication, M: From<LayershellCustomActionWithId>> MultiApplicationManager<A, M> {
  pub fn new(application: A, map_fn: fn(A::Message) -> M) -> Self {
    Self {
      application,
      windows: Vec::new(),
      map_fn,
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

  pub fn subscription(&self) -> Subscription<M>
  where
    M: 'static,
  {
    subscription::wrap_subscription(
      self.application.subscription(),
      self.windows.clone(),
      self.map_fn,
    )
  }

  pub fn remove_window(&mut self, window: window::Id) -> ControlFlow<()> {
    if let Some(index) = self
      .windows
      .iter()
      .position(|own_window| window == *own_window)
    {
      self.windows.remove(index);
      return ControlFlow::Break(());
    }

    ControlFlow::Continue(())
  }
}

impl<A, M> RequestHandler for MultiApplicationManager<A, M>
where
  A: ShellApplication,
  A: RequestHandler<Message = <A as ShellApplication>::Message>,
  M: ShellMessage + From<LayershellCustomActionWithId>,
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

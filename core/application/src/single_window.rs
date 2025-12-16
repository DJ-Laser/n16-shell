use std::ops::ControlFlow;

use iced::{Element, Subscription, Task, window};
use iced_futures::MaybeSend;
use iced_layershell::{
  actions::{LayershellCustomAction, LayershellCustomActionWithId},
  reexport::NewLayerShellSettings,
};

use n16_theme::Base16Theme;

use crate::{ShellMessage, ipc::RequestHandler, subscription};

pub enum ShellAction {
  Open(NewLayerShellSettings),
  LayershellAction(LayershellCustomAction),
  Close,
}

pub trait ShellApplication {
  type Message: ShellMessage + TryInto<ShellAction, Error = Self::Message>;

  fn update(&mut self, message: Self::Message) -> Task<Self::Message>;

  fn view(&self) -> Element<'_, Self::Message, Base16Theme>;

  fn subscription(&self) -> Subscription<Self::Message> {
    Subscription::none()
  }
}

pub struct SingleApplicationManager<A: ShellApplication, M: From<LayershellCustomActionWithId>> {
  application: A,
  window: Option<window::Id>,
  map_fn: fn(A::Message) -> M,
}

impl<A, M> SingleApplicationManager<A, M>
where
  A: ShellApplication,
  M: From<LayershellCustomActionWithId>,
{
  pub fn new(application: A, map_fn: fn(A::Message) -> M) -> Self {
    Self {
      application,
      window: None,
      map_fn,
    }
  }

  fn map_action(&mut self, action: ShellAction) -> Option<LayershellCustomActionWithId> {
    match action {
      ShellAction::Open(settings) => {
        if self.window.is_none() {
          let new_window = window::Id::unique();
          self.window = Some(new_window);

          println!("Opened window {}", new_window);

          Some(LayershellCustomActionWithId::new(
            None,
            LayershellCustomAction::NewLayerShell {
              id: new_window,
              settings,
            },
          ))
        } else {
          None
        }
      }

      ShellAction::LayershellAction(action) => {
        Some(LayershellCustomActionWithId::new(self.window, action))
      }

      ShellAction::Close => self.window.map(|window| {
        LayershellCustomActionWithId::new(Some(window), LayershellCustomAction::RemoveWindow)
      }),
    }
  }

  pub fn view(&self, window: window::Id) -> ControlFlow<Element<'_, M, Base16Theme>> {
    self
      .window
      .as_ref()
      .filter(|application_window| **application_window == window)
      .map(move |_| self.application.view().map(self.map_fn))
      .map_or(ControlFlow::Continue(()), ControlFlow::Break)
  }

  pub fn update(&mut self, message: A::Message) -> Task<M>
  where
    M: From<LayershellCustomActionWithId> + MaybeSend + 'static,
    <A as ShellApplication>::Message: 'static,
  {
    let message = match message.try_into() {
      Ok(action) => {
        return self
          .map_action(action)
          .map_or(Task::none(), |action| Task::done(M::from(action)));
      }
      Err(message) => message,
    };

    self.application.update(message).map(self.map_fn)
  }

  pub fn subscription(&self) -> Subscription<M>
  where
    M: 'static,
  {
    subscription::wrap_subscription(
      self.application.subscription(),
      self.window.into_iter().collect(),
      self.map_fn,
    )
  }

  pub fn remove_window(&mut self, window: window::Id) -> ControlFlow<()> {
    if let Some(own_window) = self.window
      && own_window == window
    {
      self.window = None;
      return ControlFlow::Break(());
    }

    ControlFlow::Continue(())
  }
}

impl<A, M> RequestHandler for SingleApplicationManager<A, M>
where
  A: ShellApplication,
  A: RequestHandler<Message = <A as ShellApplication>::Message>,
  M: ShellMessage + From<LayershellCustomActionWithId>,
{
  type Request = A::Request;
  type Message = M;

  fn handle_request(
    &mut self,
    request: Self::Request,
    reply_channel: iced::futures::channel::oneshot::Sender<n16_ipc::Reply>,
  ) -> Task<Self::Message> {
    self
      .application
      .handle_request(request, reply_channel)
      .map(self.map_fn)
  }
}

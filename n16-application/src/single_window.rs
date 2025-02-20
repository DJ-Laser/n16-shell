use std::ops::ControlFlow;

use iced::{window, Element, Subscription, Task};
use iced_futures::MaybeSend;
use iced_layershell::{
  actions::{LayershellCustomActions, LayershellCustomActionsWithId},
  reexport::NewLayerShellSettings,
};

use n16_theme::Base16Theme;

use crate::{ipc::RequestHandler, subscription, ShellMessage};

pub enum ShellAction {
  Open(NewLayerShellSettings),
  LayershellAction(LayershellCustomActions),
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

pub struct SingleApplicationManager<A: ShellApplication, M: From<LayershellCustomActionsWithId>> {
  application: A,
  window: Option<window::Id>,
  map_fn: fn(A::Message) -> M,
}

impl<A, M> SingleApplicationManager<A, M>
where
  A: ShellApplication,
  M: From<LayershellCustomActionsWithId>,
{
  pub fn new(application: A, map_fn: fn(A::Message) -> M) -> Self {
    Self {
      application,
      window: None,
      map_fn,
    }
  }

  fn map_action(&mut self, action: ShellAction) -> Option<LayershellCustomActionsWithId> {
    match action {
      ShellAction::Open(settings) => {
        if let None = self.window {
          let new_window = window::Id::unique();
          self.window = Some(new_window);

          println!("Opened window {}", new_window);

          Some(LayershellCustomActionsWithId::new(
            None,
            LayershellCustomActions::NewLayerShell {
              id: new_window,
              settings,
            },
          ))
        } else {
          println!("Open called with window {:?}", self.window);
          None
        }
      }
      ShellAction::LayershellAction(action) => {
        Some(LayershellCustomActionsWithId::new(self.window, action))
      }
      ShellAction::Close => {
        if let Some(window) = self.window {
          self.window = None;

          Some(LayershellCustomActionsWithId::new(
            None,
            LayershellCustomActions::RemoveWindow(window),
          ))
        } else {
          None
        }
      }
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
    M: From<LayershellCustomActionsWithId> + MaybeSend + 'static,
    <A as ShellApplication>::Message: 'static,
  {
    let message = match message.try_into() {
      Ok(action) => {
        return self
          .map_action(action)
          .map_or(Task::none(), |action| Task::done(M::from(action)))
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
      self.window.clone().into_iter().collect(),
      self.map_fn.clone(),
    )
  }
}

impl<A, M> RequestHandler for SingleApplicationManager<A, M>
where
  A: ShellApplication,
  A: RequestHandler<Message = <A as ShellApplication>::Message>,
  M: ShellMessage + From<LayershellCustomActionsWithId>,
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

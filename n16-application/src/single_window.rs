use std::{fmt::Debug, ops::ControlFlow};

use iced::{window, Element, Subscription, Task};
use iced_futures::MaybeSend;
use iced_layershell::{
  actions::{LayershellCustomActions, LayershellCustomActionsWithId},
  reexport::NewLayerShellSettings,
};

use n16_theme::Base16Theme;

pub enum ShellAction {
  Open(NewLayerShellSettings),
  LayershellAction(LayershellCustomActions),
  Close,
}

pub trait ShellMessage:
  TryInto<ShellAction, Error = Self> + Debug + Clone + Send + 'static
{
}

impl<T: TryInto<ShellAction, Error = Self> + Debug + Clone + Send + 'static> ShellMessage for T {}

pub trait ShellApplication {
  type Message: ShellMessage;

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

  fn map_action(&self, action: ShellAction) -> LayershellCustomActionsWithId {
    match action {
      ShellAction::Open(settings) => {
        let new_window = window::Id::unique();
        //self.window = Some(id);

        LayershellCustomActionsWithId::new(
          None,
          LayershellCustomActions::NewLayerShell {
            id: new_window,
            settings,
          },
        )
      }
      ShellAction::LayershellAction(action) => {
        LayershellCustomActionsWithId::new(self.window, action)
      }
      ShellAction::Close => LayershellCustomActionsWithId::new(
        None,
        LayershellCustomActions::RemoveWindow(
          self
            .window
            .expect("Should not call ShellAction::Close without an active window"),
        ),
      ),
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
      Ok(action) => return Task::done(M::from(self.map_action(action))),
      Err(message) => message,
    };

    self.application.update(message).map(self.map_fn)
  }

  pub fn subscription(&self) -> Subscription<A::Message> {
    self.application.subscription()
  }
}

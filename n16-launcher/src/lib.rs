use std::time::Duration;

use component::search::SEARCH_INPUT_ID;
use iced::keyboard::key;
use iced::widget::{column, container, horizontal_rule, text_input};
use iced::{Element, Length, Subscription, Task, gradient};

use component::{listing, search};
use iced_layershell::reexport::{Anchor, NewLayerShellSettings};
use listings::{Listing, Provider};
use n16_application::ipc::RequestHandler;
use n16_application::single_window::{ShellAction, ShellApplication};
use n16_ipc::launcher::{self, Request, Response};
use n16_theme::Base16Theme;
use n16_widget::scrolled_column;

mod component;
pub mod listings;
pub mod providers;

#[derive(Debug, Clone)]
pub enum Message {
  Open,
  Close,
  ListingExecuted,
  SearchQueryChanged(String),
  SelectNextListing,
  SelectPrevListing,
  RunSelected,
  ListingClicked(usize),
  FocusInput,
}

impl TryInto<ShellAction> for Message {
  type Error = Self;

  fn try_into(self) -> Result<ShellAction, Self::Error> {
    match self {
      Self::Open => Ok(ShellAction::Open(NewLayerShellSettings {
        size: Some((1000, 600)),
        anchor: Anchor::Top,
        margin: Some((200, 0, 0, 0)),
        ..Default::default()
      })),

      Self::Close => Ok(ShellAction::Close),

      _ => Err(self),
    }
  }
}

#[derive(Default)]
pub struct Launcher {
  providers: Vec<Box<dyn Provider>>,
  listings: Vec<Box<dyn Listing>>,
  filtered_listings: Vec<usize>,
  query: String,
  selected_idx: usize,
}

impl Launcher {
  pub fn new() -> Self {
    Self {
      providers: Vec::new(),
      listings: Vec::new(),
      filtered_listings: Vec::new(),
      query: String::new(),
      selected_idx: 0,
    }
  }

  pub fn add_provider<P: Provider + 'static>(&mut self, provider: P) {
    self.providers.push(Box::new(provider));
  }

  fn scroll_to_selected(&self) -> Task<Message> {
    Task::none()
  }

  fn update_listings(&mut self) {
    self.listings.clear();

    let listings = self
      .providers
      .iter_mut()
      .map(|provider| provider.update_listings())
      .flatten()
      .flatten();

    self.listings.extend(listings);
    self.filter_listings();
  }

  fn filter_listings(&mut self) {
    self.filtered_listings.clear();
    self.filtered_listings.extend(
      self
        .listings
        .iter()
        .enumerate()
        .filter(|(_idx, listing)| search::filter_listing(listing.as_ref(), &self.query))
        .map(|(idx, _listing)| idx),
    );
  }
}

impl ShellApplication for Launcher {
  type Message = Message;

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::RunSelected => {
        if let Some(listing_idx) = self.filtered_listings.get(self.selected_idx) {
          self.listings[*listing_idx].execute()
        } else {
          Task::none()
        }
      }

      Message::ListingClicked(idx) => self.listings[idx].execute(),

      Message::ListingExecuted => Task::done(Message::Close),

      Message::SearchQueryChanged(query) => {
        self.query = query;
        self.selected_idx = 0;
        self.filter_listings();
        Task::none()
      }

      Message::SelectNextListing => {
        if self.selected_idx >= self.filtered_listings.len() - 1 {
          self.selected_idx = 0;
        } else {
          self.selected_idx += 1;
        }

        self.scroll_to_selected()
      }

      Message::SelectPrevListing => {
        if self.selected_idx == 0 {
          self.selected_idx = self.filtered_listings.len() - 1;
        } else {
          self.selected_idx -= 1;
        }

        self.scroll_to_selected()
      }

      Message::FocusInput => text_input::focus(SEARCH_INPUT_ID),

      _ => Task::none(),
    }
  }

  fn view(&self) -> Element<'_, Message, Base16Theme> {
    let mut listings = scrolled_column![]
      .height(Length::Fill)
      .view_child(self.selected_idx);

    for (filtered_idx, listing_idx) in self.filtered_listings.iter().enumerate() {
      let selected = filtered_idx == self.selected_idx;
      listings = listings.push(listing::view(
        self.listings[*listing_idx].as_ref(),
        selected,
        Message::ListingClicked(*listing_idx),
      ));
    }

    let column = column![
      search::view(&self.query).into(),
      horizontal_rule(20),
      listings
    ];

    let inner = container(column)
      .height(Length::Fill)
      .padding(8)
      .style(|theme| container::Style {
        background: Some(theme.base00.into()),
        ..Default::default()
      });

    container(inner)
      .padding(4)
      .style(|theme| {
        let gradient = gradient::Linear::new(50)
          .add_stop(0.0, theme.base0D)
          .add_stop(1.0, theme.base0E);

        container::Style {
          background: Some(gradient.into()),
          ..Default::default()
        }
      })
      .into()
  }

  fn subscription(&self) -> Subscription<Message> {
    iced::event::listen_with(|event, _status, _window| match event {
      iced::Event::Window(iced::window::Event::Unfocused) => Some(Message::Close),
      iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => match key {
        iced::keyboard::Key::Named(key::Named::ArrowUp) => Some(Message::SelectPrevListing),
        iced::keyboard::Key::Named(key::Named::ArrowDown) => Some(Message::SelectNextListing),
        iced::keyboard::Key::Named(key::Named::Enter) => Some(Message::RunSelected),
        iced::keyboard::Key::Named(key::Named::Escape) => Some(Message::Close),
        _ => None,
      },
      _ => None,
    })
  }
}

impl RequestHandler for Launcher {
  type Request = launcher::Request;
  type Message = Message;

  fn handle_request(
    &mut self,
    request: Self::Request,
    reply_channel: iced::futures::channel::oneshot::Sender<n16_ipc::Reply>,
  ) -> Task<Self::Message> {
    match request {
      Request::Open => {
        let _ = reply_channel.send(Response::handled().reply_ok());
        self.update_listings();

        Task::done(Message::Open).chain(Task::future(async {
          tokio::time::sleep(Duration::from_millis(250)).await;
          Message::FocusInput
        }))
      }
      Request::Close => {
        let _ = reply_channel.send(Response::handled().reply_ok());
        Task::done(Message::Close)
      }
    }
  }
}

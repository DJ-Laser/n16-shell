use std::mem;
use std::sync::Arc;
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
use tokio::sync::Mutex;

mod component;
pub mod listings;
pub mod providers;

type Providers = Arc<Mutex<Vec<Box<dyn Provider>>>>;
type Listings = Vec<Box<dyn Listing>>;

#[derive(Debug, Clone)]
pub enum Message {
  OpenLayerShell,
  CloseLayerShell,
  Open,
  ListingExecuted,
  SearchQueryChanged(String),
  SelectNextListing,
  SelectPrevListing,
  RunSelected,
  ListingClicked(usize),
  FocusInput,
  UpdatedListings(Listings),
}

impl TryInto<ShellAction> for Message {
  type Error = Self;

  fn try_into(self) -> Result<ShellAction, Self::Error> {
    match self {
      Self::OpenLayerShell => Ok(ShellAction::Open(NewLayerShellSettings {
        size: Some((1000, 600)),
        anchor: Anchor::Top,
        margin: Some((200, 0, 0, 0)),
        ..Default::default()
      })),

      Self::CloseLayerShell => Ok(ShellAction::Close),

      _ => Err(self),
    }
  }
}

#[derive(Default)]
pub struct Launcher {
  providers: Providers,
  listings: Listings,
  filtered_listings: Vec<usize>,
  query: String,
  selected_idx: usize,
}

impl Launcher {
  pub fn new() -> Self {
    Self {
      providers: Default::default(),
      listings: Vec::new(),
      filtered_listings: Vec::new(),
      query: String::new(),
      selected_idx: 0,
    }
  }

  pub fn add_provider<P: Provider + 'static>(&mut self, provider: P) {
    let mut providers = self.providers.blocking_lock();
    providers.push(Box::new(provider));
  }

  pub async fn add_provider_async<P: Provider + 'static>(&mut self, provider: P) {
    let mut providers = self.providers.lock().await;
    providers.push(Box::new(provider));
  }

  fn scroll_to_selected(&self) -> Task<Message> {
    Task::none()
  }

  async fn update_listings(providers: Providers) -> Listings {
    let mut providers = providers.lock().await;

    let listings = providers
      .iter_mut()
      .map(|provider| provider.update_listings())
      .flatten()
      .flatten();

    listings.collect()
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

  fn update_query(&mut self, new_query: &str) {
    self.query.clear();
    self.query.push_str(new_query);
    self.selected_idx = 0;
    self.filter_listings();
  }
}

impl ShellApplication for Launcher {
  type Message = Message;

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Open => {
        self.update_query("");

        Task::batch([
          Task::future(async {
            tokio::time::sleep(Duration::from_millis(250)).await;
            Message::FocusInput
          }),
          Task::perform(
            Self::update_listings(self.providers.clone()),
            Message::UpdatedListings,
          ),
        ])
      }

      Message::RunSelected => {
        if let Some(listing_idx) = self.filtered_listings.get(self.selected_idx) {
          self.listings[*listing_idx].execute()
        } else {
          Task::none()
        }
      }

      Message::ListingClicked(idx) => self.listings[idx].execute(),

      Message::ListingExecuted => Task::done(Message::CloseLayerShell),

      Message::SearchQueryChanged(new_query) => {
        self.update_query(&new_query);
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

      Message::UpdatedListings(new_listings) => {
        let _ = mem::replace(&mut self.listings, new_listings);
        self.filter_listings();

        Task::none()
      }

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
      iced::Event::Window(iced::window::Event::Unfocused) => Some(Message::CloseLayerShell),
      iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => match key {
        iced::keyboard::Key::Named(key::Named::ArrowUp) => Some(Message::SelectPrevListing),
        iced::keyboard::Key::Named(key::Named::ArrowDown) => Some(Message::SelectNextListing),
        iced::keyboard::Key::Named(key::Named::Enter) => Some(Message::RunSelected),
        iced::keyboard::Key::Named(key::Named::Escape) => Some(Message::CloseLayerShell),
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

        Task::batch([
          Task::done(Message::OpenLayerShell),
          Task::done(Message::Open),
        ])
      }
      Request::Close => {
        let _ = reply_channel.send(Response::handled().reply_ok());
        Task::done(Message::CloseLayerShell)
      }
    }
  }
}

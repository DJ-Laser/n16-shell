use iced::{
  Element, Length, Subscription, Task, gradient,
  keyboard::key,
  widget::{column, container, operation, rule, text},
  window,
};
use iced_layershell::{
  reexport::{Anchor, NewLayerShellSettings},
  settings::{LayerShellSettings, StartMode},
  to_layer_message,
};
use listings::{Listing, Provider};
use n16_core::{
  application::{ApplicationRequest, N16Application, RequestChannel},
  scrolled_column,
  theme::{self, Base16Theme},
};
use n16_ipc::{Response, launcher::Request};
use std::{mem, sync::Arc};
use tokio::sync::Mutex;

use crate::{
  calculator::Calculator,
  component::{
    listing,
    search::{self, SEARCH_INPUT_ID},
  },
};

mod calculator;
mod component;
pub mod listings;
pub mod providers;

type Providers = Arc<Mutex<Vec<Box<dyn Provider>>>>;
type Listings = Vec<Box<dyn Listing>>;

pub struct Launcher {
  calculator: Calculator,
  calculator_result: Option<String>,

  providers: Providers,
  listings: Listings,
  filtered_listings: Vec<usize>,
  query: String,
  selected_idx: usize,
}

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
  Close,
  ListingExecuted,
  SearchQueryChanged(String),
  SelectNextListing,
  SelectPrevListing,
  RunSelected,
  ListingClicked(usize),
  FocusInput,
  UpdatedListings(Listings),
  CalculatorResult(Option<String>),
  RequestRecieved(ApplicationRequest<Request>),
}

impl Launcher {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self {
      calculator: Default::default(),
      calculator_result: None,

      providers: Default::default(),
      listings: Vec::new(),
      filtered_listings: Vec::new(),
      query: String::new(),
      selected_idx: 0,
    }
  }

  fn scroll_to_selected(&self) -> Task<Message> {
    Task::none()
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

  fn update_query(&mut self, new_query: &str) -> Task<Message> {
    self.query.clear();
    self.query.push_str(new_query);
    self.selected_idx = 0;
    self.filter_listings();

    Task::done(self.calculator.calculate_preview(&self.query)).map(Message::CalculatorResult)
  }

  async fn add_provider<P: Provider + 'static>(&mut self, provider: P) {
    let mut providers = self.providers.lock().await;
    providers.push(Box::new(provider));
  }

  async fn update_listings(&mut self) -> Listings {
    let mut providers = self.providers.lock().await;

    let listings = providers
      .iter_mut()
      .filter_map(|provider| provider.update_listings())
      .flatten();

    listings.collect()
  }
}

impl Launcher {
  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::RequestRecieved(request) => match request.kind() {
        Request::Open => {
          request.reply(Response::Handled);
          Message::layershell_open(NewLayerShellSettings {
            size: Some((1000, 600)),
            anchor: Anchor::Top,
            margin: Some((200, 0, 0, 0)),
            ..Default::default()
          })
          .1
        }
        Request::Close => {
          request.reply(Response::Handled);
          Task::done(Message::Close)
        }
      },

      Message::Close => window::latest().and_then(window::close),

      Message::RunSelected => {
        if let Some(listing_idx) = self.filtered_listings.get(self.selected_idx) {
          self.listings[*listing_idx].execute()
        } else {
          Task::none()
        }
      }

      Message::ListingClicked(idx) => self.listings[idx].execute(),

      Message::ListingExecuted => Task::done(Message::Close),

      Message::SearchQueryChanged(new_query) => self.update_query(&new_query),

      Message::SelectNextListing => {
        if self.filtered_listings.is_empty()
          || self.selected_idx >= self.filtered_listings.len() - 1
        {
          self.selected_idx = 0;
        } else {
          self.selected_idx += 1;
        }

        self.scroll_to_selected()
      }

      Message::SelectPrevListing => {
        if self.filtered_listings.is_empty() {
          self.selected_idx = 0;
        } else if self.selected_idx == 0 {
          self.selected_idx = self.filtered_listings.len() - 1;
        } else {
          self.selected_idx -= 1;
        }

        self.scroll_to_selected()
      }

      Message::FocusInput => operation::focus(SEARCH_INPUT_ID),

      Message::UpdatedListings(new_listings) => {
        let _ = mem::replace(&mut self.listings, new_listings);
        self.filter_listings();

        Task::none()
      }

      Message::CalculatorResult(result) => {
        self.calculator_result = result;
        Task::none()
      }

      _ => unreachable!(),
    }
  }

  pub fn view(&self, _id: window::Id) -> Element<'_, Message, Base16Theme> {
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

    let mut column = column![search::view(&self.query).into()];

    if let Some(result) = &self.calculator_result {
      column = column.push(
        column![
          rule::horizontal(1).style(|theme: &Base16Theme| theme::rule::colored(theme.base02))
        ]
        .height(20),
      );
      column = column.push(text(result));
    }

    column = column.push(
      column![rule::horizontal(1).style(|theme: &Base16Theme| theme::rule::colored(theme.base02))]
        .height(20),
    );
    column = column.push(listings);

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

  pub fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([iced::event::listen_with(
      |event, _status, _window| match event {
        iced::Event::Window(iced::window::Event::Unfocused) => Some(Message::Close),
        iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => match key {
          iced::keyboard::Key::Named(key::Named::ArrowUp) => Some(Message::SelectPrevListing),
          iced::keyboard::Key::Named(key::Named::ArrowDown) => Some(Message::SelectNextListing),
          iced::keyboard::Key::Named(key::Named::Enter) => Some(Message::RunSelected),
          iced::keyboard::Key::Named(key::Named::Escape) => Some(Message::Close),
          _ => None,
        },
        _ => None,
      },
    )])
  }
}

impl N16Application for Launcher {
  type Request = Request;

  fn run(request_rx: RequestChannel<Self::Request>) {
    let _ = iced_layershell::daemon(
      move || {
        (
          Launcher::new(),
          Task::batch([
            Task::stream(request_rx.clone()).map(Message::RequestRecieved),
            Task::done(Message::FocusInput),
          ]),
        )
      },
      "n16_launcher",
      Launcher::update,
      Launcher::view,
    )
    .layer_settings(LayerShellSettings {
      start_mode: StartMode::Background,
      ..Default::default()
    })
    .subscription(Launcher::subscription)
    .run();
  }
}

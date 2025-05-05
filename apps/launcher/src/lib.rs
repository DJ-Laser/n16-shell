use std::mem;
use std::sync::Arc;
use std::time::Duration;

use calculator::Calculator;
use component::search::SEARCH_INPUT_ID;
use iced::keyboard::key;
use iced::widget::{column, container, horizontal_rule, text, text_input};
use iced::{Element, Length, Subscription, Task, gradient};

use component::{listing, search};
use iced_layershell::reexport::{Anchor, NewLayerShellSettings};
use listings::{Listing, Provider};
use n16_application::ipc::RequestHandler;
use n16_application::single_window::{ShellAction, ShellApplication};
use n16_ipc::launcher::{self, Request, Response};
use n16_theme::{Base16Theme, rule};
use n16_widget::scrolled_column;
use tokio::sync::Mutex;

pub mod calculator;
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
  CalculatorResult(Option<String>),
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
  calculator: Calculator,
  calculator_result: Option<String>,

  providers: Providers,
  listings: Listings,
  filtered_listings: Vec<usize>,
  query: String,
  selected_idx: usize,
}

impl Launcher {
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

  fn update_query(&mut self, new_query: &str) -> Task<Message> {
    self.query.clear();
    self.query.push_str(new_query);
    self.selected_idx = 0;
    self.filter_listings();

    Task::done(self.calculator.calculate_preview(&self.query)).map(Message::CalculatorResult)
  }
}

impl ShellApplication for Launcher {
  type Message = Message;

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Open => Task::batch([
        self.update_query(""),
        Task::future(async {
          tokio::time::sleep(Duration::from_millis(250)).await;
          Message::FocusInput
        }),
        Task::perform(
          Self::update_listings(self.providers.clone()),
          Message::UpdatedListings,
        ),
      ]),

      Message::RunSelected => {
        if let Some(listing_idx) = self.filtered_listings.get(self.selected_idx) {
          self.listings[*listing_idx].execute()
        } else {
          Task::none()
        }
      }

      Message::ListingClicked(idx) => self.listings[idx].execute(),

      Message::ListingExecuted => Task::done(Message::CloseLayerShell),

      Message::SearchQueryChanged(new_query) => self.update_query(&new_query),

      Message::SelectNextListing => {
        if self.filtered_listings.len() == 0 {
          self.selected_idx = 0;
        } else if self.selected_idx >= self.filtered_listings.len() - 1 {
          self.selected_idx = 0;
        } else {
          self.selected_idx += 1;
        }

        self.scroll_to_selected()
      }

      Message::SelectPrevListing => {
        if self.filtered_listings.len() == 0 {
          self.selected_idx = 0;
        } else if self.selected_idx == 0 {
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

      Message::CalculatorResult(result) => {
        self.calculator_result = result;
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

    let mut column = column![search::view(&self.query).into()];

    if let Some(result) = &self.calculator_result {
      column =
        column.push(horizontal_rule(20).style(|theme: &Base16Theme| rule::colored(theme.base02)));
      column = column.push(text(result));
    }

    column = column.push(horizontal_rule(20));
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

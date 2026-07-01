use std::mem;

use crate::Listings;
use crate::calculator::Calculator;
use crate::component::search::SEARCH_INPUT_ID;
use iced::keyboard::key;
use iced::widget::{column, container, operation, rule, text};
use iced::{Element, Length, Subscription, Task, gradient};

use crate::component::{listing, search};
use n16_theme::Base16Theme;
use n16_widget::scrolled_column;

pub struct Launcher {
  calculator: Calculator,
  calculator_result: Option<String>,

  listings: Listings,
  filtered_listings: Vec<usize>,
  query: String,
  selected_idx: usize,
}

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
}

impl Launcher {
  pub fn new(listings: Listings) -> Self {
    Self {
      calculator: Default::default(),
      calculator_result: None,

      listings,
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
}

impl Launcher {
  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Close => iced::exit(),

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
    }
  }

  pub fn view(&self) -> Element<'_, Message, Base16Theme> {
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
          rule::horizontal(1).style(|theme: &Base16Theme| n16_theme::rule::colored(theme.base02))
        ]
        .height(20),
      );
      column = column.push(text(result));
    }

    column = column.push(
      column![
        rule::horizontal(1).style(|theme: &Base16Theme| n16_theme::rule::colored(theme.base02))
      ]
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

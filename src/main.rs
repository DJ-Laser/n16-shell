use std::fmt::Debug;

use component::search::SEARCH_INPUT_ID;
use iced::keyboard::key;
use iced::widget::{column, container, horizontal_rule, text_input};
use iced::{color, gradient, Element, Length, Subscription, Task};
use iced_layershell::build_pattern::{application, MainSettings};
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::to_layer_message;

use component::{listing, search};
use listings::{Listing, Provider};
use providers::applications::ApplicationProvider;
use providers::power_management::PowerManagementProvider;
use theme::Base16Theme;

mod component;
pub mod listings;
pub mod providers;
pub mod theme;
mod widget;

fn main() -> Result<(), iced_layershell::Error> {
  let theme = Base16Theme {
    base00: color!(0x1d1f21),
    base01: color!(0x282a2e),
    base02: color!(0x373b41),
    base03: color!(0x969896),
    base04: color!(0xb4b7b4),
    base05: color!(0xc5c8c6),
    base06: color!(0xe0e0e0),
    base07: color!(0xffffff),
    base08: color!(0xcc6666),
    base09: color!(0xde935f),
    base0A: color!(0xf0c674),
    base0B: color!(0xb5bd68),
    base0C: color!(0x8abeb7),
    base0D: color!(0x81a2be),
    base0E: color!(0xb294bb),
    base0F: color!(0xa3685a),
  };

  let mut launcher = Launcher::new();
  launcher.add_provider(ApplicationProvider::new());
  launcher.add_provider(PowerManagementProvider::new());

  application("A cool counter", Launcher::update, Launcher::view)
    .subscription(Launcher::subscription)
    .theme(move |_| theme.clone())
    .settings(MainSettings {
      layer_settings: LayerShellSettings {
        size: Some((1000, 600)),
        anchor: Anchor::Top,
        margin: (200, 0, 0, 0),
        ..Default::default()
      },
      ..Default::default()
    })
    .run_with(|| (launcher, Task::done(Message::Init)))
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
  Init,
  ListingExecuted,
  SearchQueryChanged(String),
  SelectNextListing,
  SelectPrevListing,
  RunSelected,
  ListingClicked(usize),
  Exit,
}

#[derive(Default)]
struct Launcher {
  providers: Vec<Box<dyn Provider>>,
  listings: Vec<Box<dyn Listing>>,
  filtered_listings: Vec<usize>,
  query: String,
  selected_idx: usize,
}

impl Launcher {
  fn new() -> Self {
    Self {
      providers: Vec::new(),
      listings: Vec::new(),
      filtered_listings: Vec::new(),
      query: String::new(),
      selected_idx: 0,
    }
  }

  fn add_provider<P: Provider + 'static>(&mut self, provider: P) {
    self.providers.push(Box::new(provider));
  }

  fn subscription(&self) -> Subscription<Message> {
    iced::event::listen_with(|event, _status, _window| match event {
      iced::Event::Window(iced::window::Event::Unfocused) => Some(Message::Exit),
      iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => match key {
        iced::keyboard::Key::Named(key::Named::ArrowUp) => Some(Message::SelectPrevListing),
        iced::keyboard::Key::Named(key::Named::ArrowDown) => Some(Message::SelectNextListing),
        iced::keyboard::Key::Named(key::Named::Enter) => Some(Message::RunSelected),
        iced::keyboard::Key::Named(key::Named::Escape) => Some(Message::Exit),
        _ => None,
      },
      _ => None,
    })
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

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Init => {
        self.update_listings();
        text_input::focus(SEARCH_INPUT_ID)
      }

      Message::RunSelected => self.listings[self.filtered_listings[self.selected_idx]].execute(),

      Message::ListingClicked(idx) => self.listings[idx].execute(),

      Message::ListingExecuted => Task::done(Message::Exit),

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

      Message::Exit => iced_runtime::task::effect(iced_runtime::Action::Exit),

      _ => todo!(),
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
}

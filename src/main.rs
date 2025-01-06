use component::search::{preprocess_query, SEARCH_INPUT_ID};
use iced::keyboard::key;
use iced::widget::scrollable::AbsoluteOffset;
use iced::widget::{column, container, horizontal_rule, scrollable, text_input};
use iced::{color, gradient, Element, Length, Subscription, Task};
use iced_layershell::build_pattern::{application, MainSettings};
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::to_layer_message;

use component::{listing, search};
use listings::applications::ApplicationProvider;
use listings::{Listing, Provider};
use theme::Base16Theme;

mod component;
pub mod listings;
pub mod theme;

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
    .run_with(|| (Launcher::new(), Task::done(Message::Init)))
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
  Init,
  ListingClicked(usize),
  SearchQueryChanged(String),
  SelectNextListing,
  SelectPrevListing,
  RunSelected,
  Exit,
}

#[derive(Default)]
struct Launcher {
  provider: ApplicationProvider,
  listings: Vec<Listing>,
  query: String,
  selected_idx: usize,
}

impl Launcher {
  fn new() -> Self {
    Self {
      provider: ApplicationProvider::new(),
      listings: Vec::new(),
      query: String::new(),
      selected_idx: 0,
    }
  }

  fn subscription(&self) -> Subscription<Message> {
    iced::event::listen_with(|event, _status, _window| match event {
      iced::Event::Window(iced::window::Event::Unfocused) => Some(Message::Exit),
      iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => match key {
        iced::keyboard::Key::Named(key::Named::ArrowUp) => Some(Message::SelectPrevListing),
        iced::keyboard::Key::Named(key::Named::ArrowDown) => Some(Message::SelectNextListing),
        iced::keyboard::Key::Named(key::Named::Enter) => Some(Message::RunSelected),
        _ => None,
      },
      _ => None,
    })
  }

  const SCROLLABLE_ID: &'static str = "LISTING_SCROLL_AREA";

  fn scroll_to_selected(&self) -> Task<Message> {
    scrollable::scroll_to(
      scrollable::Id::new(Self::SCROLLABLE_ID),
      AbsoluteOffset {
        x: 0.0,
        y: (40 * self.selected_idx) as f32,
      },
    )
  }

  fn filter_listings(&mut self) {
    self.listings.clear();

    let listings =
      self.provider.listings().into_iter().filter(|listing| {
        preprocess_query(listing.name()).contains(&preprocess_query(&self.query))
      });

    self.listings.extend(listings);
  }

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Init => {
        self.provider.update_listings();
        self.filter_listings();
        text_input::focus(SEARCH_INPUT_ID)
      }

      Message::ListingClicked(idx) => {
        self.provider.execute(idx);
        Task::done(Message::Exit)
      }

      Message::RunSelected => {
        self.provider.execute(self.listings[self.selected_idx].id());
        Task::done(Message::Exit)
      }

      Message::SearchQueryChanged(query) => {
        self.query = query;
        self.selected_idx = 0;
        self.filter_listings();
        Task::none()
      }

      Message::SelectNextListing => {
        if self.selected_idx >= self.listings.len() - 1 {
          self.selected_idx = 0;
        } else {
          self.selected_idx += 1;
        }

        self.scroll_to_selected()
      }

      Message::SelectPrevListing => {
        if self.selected_idx == 0 {
          self.selected_idx = self.listings.len() - 1;
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
    let mut listings = column![];

    for (index, listing) in self.listings.clone().into_iter().enumerate() {
      let selected = index == self.selected_idx;
      let listing_id = listing.id();
      listings = listings.push(listing::view(
        listing,
        selected,
        Message::ListingClicked(listing_id),
      ));
    }

    let listings_container: container::Container<'_, Message, Base16Theme> = container(
      scrollable(listings)
        .id(scrollable::Id::new(Self::SCROLLABLE_ID))
        .direction(scrollable::Direction::Vertical(
          scrollable::Scrollbar::default().width(0).scroller_width(0),
        )),
    );

    let column = column![
      search::view(&self.query).into(),
      horizontal_rule(20),
      listings_container
    ];

    let inner = container(column)
      .height(Length::Fill)
      .padding(10)
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

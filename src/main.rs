use component::search::{preprocess_query, SEARCH_INPUT_ID};
use iced::widget::{column, container, horizontal_rule, scrollable, text_input};
use iced::{color, gradient, Element, Length, Subscription, Task};
use iced_layershell::build_pattern::{application, MainSettings};
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::to_layer_message;

use component::{listing, search};
use listings::applications::ApplicationProvider;
use listings::Provider;
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
  Exit,
}

#[derive(Default)]
struct Launcher {
  provider: ApplicationProvider,
  query: String,
}

impl Launcher {
  fn new() -> Self {
    Self {
      provider: ApplicationProvider::new(),
      query: String::new(),
    }
  }

  fn subscription(&self) -> Subscription<Message> {
    iced::event::listen_with(|event, _status, _window| match event {
      iced::Event::Window(iced::window::Event::Unfocused) => Some(Message::Exit),
      _ => None,
    })
  }

  fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Init => {
        self.provider.update_listings();
        text_input::focus(SEARCH_INPUT_ID)
      }

      Message::ListingClicked(idx) => {
        self.provider.execute(idx);
        Task::done(Message::Exit)
      }

      Message::SearchQueryChanged(query) => {
        self.query = query;
        Task::none()
      }

      Message::Exit => iced_runtime::task::effect(iced_runtime::Action::Exit),

      _ => todo!(),
    }
  }

  fn view(&self) -> Element<'_, Message, Base16Theme> {
    let apps = self
      .provider
      .listings()
      .into_iter()
      .enumerate()
      .filter(|(_idx, listing)| {
        preprocess_query(listing.name()).contains(&preprocess_query(&self.query))
      });
    let mut listings = column![];

    for (idx, listing) in apps.into_iter() {
      listings = listings.push(listing::view(listing, Message::ListingClicked(idx)));
    }

    let listings_container: container::Container<'_, Message, Base16Theme> = container(
      scrollable(listings).direction(scrollable::Direction::Vertical(
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

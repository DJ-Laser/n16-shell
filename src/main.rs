use iced::futures::io::Window;
use iced::widget::{column, scrollable};
use iced::{Element, Subscription, Task, Theme};
use iced_layershell::build_pattern::{application, MainSettings};
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::to_layer_message;

use component::listing;
use listings::applications::ApplicationProvider;
use listings::Provider;

mod component;
pub mod listings;

fn main() -> Result<(), iced_layershell::Error> {
  application("A cool counter", Launcher::update, Launcher::view)
    .subscription(Launcher::subscription)
    .theme(|_| Theme::Dark)
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
#[derive(Debug, Clone, Copy)]
enum Message {
  Init,
  ListingClicked(usize),
  Exit,
}

#[derive(Default)]
struct Launcher {
  provider: ApplicationProvider,
}

impl Launcher {
  fn new() -> Self {
    Self {
      provider: ApplicationProvider::new(),
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
        Task::none()
      }

      Message::ListingClicked(idx) => {
        self.provider.execute(idx);
        Task::done(Message::Exit)
      }

      Message::Exit => iced_runtime::task::effect(iced_runtime::Action::Exit),

      _ => todo!(),
    }
  }

  fn view(&self) -> Element<'_, Message> {
    let apps = self.provider.listings();
    let mut listings = column![];

    for (idx, listing) in apps.into_iter().enumerate() {
      listings = listings.push(listing::view(listing, Message::ListingClicked(idx)));
    }

    scrollable(listings)
      .direction(scrollable::Direction::Vertical(
        scrollable::Scrollbar::default().width(0).scroller_width(0),
      ))
      .into()
  }
}

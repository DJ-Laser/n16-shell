use iced::widget::{column, scrollable};
use iced::{Element, Task, Theme};
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
  application("A cool counter", Counter::update, Counter::view)
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
    .run_with(|| (Counter::new(), Task::none()))
}

#[to_layer_message]
#[derive(Debug, Clone, Copy)]
enum Message {
  Init,
}

#[derive(Default)]
struct Counter {
  provider: ApplicationProvider,
}

impl Counter {
  fn new() -> Self {
    Self {
      provider: ApplicationProvider::new(),
    }
  }

  fn update(&mut self, message: Message) {
    match message {
      Message::Init => {
        self.provider.update_listings();
      }

      _ => todo!(),
    }
  }

  fn view(&self) -> Element<'_, Message> {
    let mut provider = ApplicationProvider::new();
    provider.update_listings();

    let apps = provider.listings();
    let mut lines = column![];

    for listing in apps {
      lines = lines.push(listing::view(listing));
    }

    scrollable(lines).into()
  }
}

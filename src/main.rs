use iced::{color, window, Element, Subscription, Task};
use iced_layershell::build_pattern::MainSettings;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::{build_pattern::daemon, to_layer_message};

use n16_launcher::providers::{ApplicationProvider, PowerManagementProvider};
use n16_launcher::Launcher;
use n16_theme::Base16Theme;

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

  let shell = Shell::new(launcher);

  daemon("N16 Shell", Shell::update, Shell::view, Shell::remove_id)
    .subscription(Shell::subscription)
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
    .run_with(|| (shell, Task::done(Message::Init)))
}

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
enum Message {
  Init,
  Launcher(n16_launcher::Message),
}

struct Shell {
  launcher: (Launcher, Option<window::Id>),
}

impl Shell {
  pub fn new(launcher: Launcher) -> Self {
    Self {
      launcher: (launcher, None),
    }
  }

  pub fn view(&self, window: window::Id) -> Element<'_, Message, Base16Theme> {
    self.launcher.0.view().map(Message::Launcher)
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Init => self
        .launcher
        .0
        .update(n16_launcher::Message::Open)
        .map(Message::Launcher),

      Message::Launcher(launcher_message) => self
        .launcher
        .0
        .update(launcher_message)
        .map(Message::Launcher),

      _ => Task::none(),
    }
  }

  pub fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([self.launcher.0.subscription().map(Message::Launcher)])
  }

  pub fn remove_id(&mut self, window: window::Id) {
    println!("remove_id called with window {:?}", window);
  }
}

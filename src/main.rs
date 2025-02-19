use std::ops::ControlFlow;

use iced::{color, window, Element, Subscription, Task};
use iced_layershell::build_pattern::daemon;
use iced_layershell::build_pattern::MainSettings;
use iced_layershell::settings::{LayerShellSettings, StartMode};

use n16_application::single_window::SingleApplicationManager;
use n16_launcher::providers::{ApplicationProvider, PowerManagementProvider};
use n16_launcher::Launcher;
use n16_theme::Base16Theme;

pub use message::Message;

mod message;

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
        start_mode: StartMode::Background,
        ..Default::default()
      },
      ..Default::default()
    })
    .run_with(|| (shell, Task::done(Message::Init)))
}

struct Shell {
  launcher: SingleApplicationManager<Launcher, Message>,
}

impl Shell {
  pub fn new(launcher: Launcher) -> Self {
    Self {
      launcher: SingleApplicationManager::new(launcher, Message::Launcher),
    }
  }

  fn try_view(&self, window: window::Id) -> ControlFlow<Element<'_, Message, Base16Theme>> {
    self.launcher.view(window)?;
    ControlFlow::Continue(())
  }

  pub fn view(&self, window: window::Id) -> Element<'_, Message, Base16Theme> {
    self
      .try_view(window)
      .break_value()
      .expect("Shell::view should not be called with an unclaimed window")
  }

  pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
      Message::Init => Task::none(),

      Message::Launcher(launcher_message) => self.launcher.update(launcher_message),

      _ => Task::none(),
    }
  }

  pub fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([self.launcher.subscription().map(Message::Launcher)])
  }

  pub fn remove_id(&mut self, window: window::Id) {
    println!("remove_id called with window {:?}", window);
  }
}

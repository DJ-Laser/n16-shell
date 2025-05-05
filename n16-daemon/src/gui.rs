use iced::color;
use iced_layershell::build_pattern::MainSettings;
use iced_layershell::build_pattern::daemon;
use iced_layershell::settings::{LayerShellSettings, StartMode};

use n16_bar::Bar;
use n16_launcher::Launcher;
use n16_launcher::providers::{ApplicationProvider, PowerManagementProvider};
use n16_theme::Base16Theme;

pub use message::*;
pub use shell::*;

mod message;
mod shell;

pub fn run_iced_daemon() -> Result<(), iced_layershell::Error> {
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

  let bar = Bar::new();

  let shell = Shell::new(launcher, bar);

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
    .run_with(|| shell)
}

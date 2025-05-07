use iced_layershell::build_pattern::MainSettings;
use iced_layershell::build_pattern::daemon;
use iced_layershell::settings::{LayerShellSettings, StartMode};

use n16_bar::Bar;
use n16_launcher::Launcher;
use n16_launcher::providers::{ApplicationProvider, PowerManagementProvider};

pub use message::*;
pub use shell::*;

mod message;
mod shell;

pub fn run_iced_daemon() -> Result<(), iced_layershell::Error> {
  let mut launcher = Launcher::new();
  launcher.add_provider(ApplicationProvider::new());
  launcher.add_provider(PowerManagementProvider::new());

  let bar = Bar::new();

  let shell = Shell::new(launcher, bar);

  daemon("N16 Shell", Shell::update, Shell::view, Shell::remove_id)
    .subscription(Shell::subscription)
    .theme(Shell::theme)
    .settings(MainSettings {
      layer_settings: LayerShellSettings {
        start_mode: StartMode::Background,
        ..Default::default()
      },
      ..Default::default()
    })
    .run_with(|| shell)
}

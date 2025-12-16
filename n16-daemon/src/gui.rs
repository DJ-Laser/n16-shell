use iced_layershell::settings::{LayerShellSettings, StartMode};
use iced_layershell::{Settings, daemon};

pub use message::*;
pub use shell::*;

mod message;
mod shell;

pub fn run_iced_daemon() -> Result<(), iced_layershell::Error> {
  daemon(Shell::new, "N16 Shell", Shell::update, Shell::view)
    .subscription(Shell::subscription)
    .theme(|shell: &Shell, _| shell.theme())
    .settings(Settings {
      layer_settings: LayerShellSettings {
        start_mode: StartMode::Background,
        ..Default::default()
      },
      ..Default::default()
    })
    .run()
}

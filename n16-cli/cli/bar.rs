use clap::{Parser, Subcommand};
use n16_ipc::bar::Request;

#[derive(Parser, Clone, Debug)]

/// Hide and show the status bar
pub struct Cli {
  #[command(subcommand)]
  command: Command,
}

impl Cli {
  pub fn request(&self) -> n16_ipc::Request {
    let bar_request = match self.command {
      Command::Show => Request::Show,
      Command::Hide => Request::Hide,
    };

    n16_ipc::Request::Bar(bar_request)
  }
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
  /// Show the status bar
  Show,
  /// Hide the status bar
  Hide,
}

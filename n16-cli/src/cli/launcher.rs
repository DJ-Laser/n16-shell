use clap::{Parser, Subcommand};
use n16_ipc::launcher::Request;

#[derive(Parser, Clone, Debug)]

/// Open and close the app launcher
pub struct Cli {
  #[command(subcommand)]
  command: Command,
}

impl Cli {
  pub fn request(&self) -> n16_ipc::Request {
    let launcher_request = match self.command {
      Command::Open => Request::Open,
      Command::Close => Request::Close,
    };

    n16_ipc::Request::Launcher(launcher_request)
  }
}

#[derive(Subcommand, Clone, Debug)]
enum Command {
  /// Open the app launcher window
  Open,
  /// Close the app launcher window
  Close,
}

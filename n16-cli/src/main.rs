use clap::Parser;
use cli::Cli;
use ipc::send_request_ok;

mod cli;
mod ipc;
pub fn main() {
  let cli = Cli::parse();

  match cli.command {
    cli::Command::Launcher(launcher) => {
      send_request_ok(launcher.request());
    }

    cli::Command::Bar(bar) => {
      send_request_ok(bar.request());
    }
  };
}

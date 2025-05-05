use clap::{Parser, Subcommand};

mod bar;
mod launcher;

#[derive(Parser, Debug)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  Launcher(launcher::Cli),
  Bar(bar::Cli),
}

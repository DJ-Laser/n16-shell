use std::env::args;

use daemon::run_daemon;
use n16_ipc::{launcher, Request};

mod daemon;

pub fn main() {
  match args().nth(1) {
    Some(arg) if arg == "--daemon" => run_daemon(),
    Some(arg) => panic!("Unexpected arg {}", arg),
    None => (),
  }

  print!("{}", Request::Launcher(launcher::Request::Open).to_json())
}

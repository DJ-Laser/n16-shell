use std::env::args;

use n16_ipc::{launcher, Request};

pub fn main() {
  match args().nth(1) {
    Some(arg) if arg == "--daemon" => n16_daemon::run_daemon(),
    Some(arg) => panic!("Unexpected arg {}", arg),
    None => (),
  }

  print!("{}", Request::Launcher(launcher::Request::Open).to_json())
}

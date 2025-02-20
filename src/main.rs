use std::env::args;

use ipc::send_request;
use n16_ipc::{launcher, Request};

mod ipc;

pub fn main() {
  match args().nth(1) {
    Some(arg) if arg == "--daemon" => n16_daemon::run_daemon(),
    Some(arg) => panic!("Unexpected arg {}", arg),
    None => (),
  };

  send_request(Request::Launcher(launcher::Request::Open))
    .unwrap()
    .unwrap();
}

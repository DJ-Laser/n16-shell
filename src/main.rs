use std::env::args;

use ipc::send_request;
use n16_ipc::{Request, launcher};

mod ipc;

pub fn main() {
  send_request(Request::Launcher(launcher::Request::Open))
    .unwrap()
    .unwrap();
}

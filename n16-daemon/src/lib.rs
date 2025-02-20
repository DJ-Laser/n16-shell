use std::process;

use gui::run_iced_daemon;

mod gui;
mod ipc;

pub fn run_daemon() -> ! {
  loop {
    // The iced daemon should never exit unless an error has occured
    let res = run_iced_daemon();

    match res {
      Ok(()) => eprintln!("Daemon restarting due to error in a shell application"),
      Err(err) => {
        eprintln!("Daemon stopping due to internal iced error: {:?}", err);
        process::exit(1)
      }
    }
  }
}

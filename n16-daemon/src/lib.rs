use std::{ops::ControlFlow, pin::pin, process};

use futures_lite::StreamExt;
use n16_bar::Bar;
use n16_ipc::{Request, Response};
use n16_launcher::Launcher;

use crate::{application::run_application, ipc::run_ipc_server};

mod application;
mod ipc;

pub async fn run_daemon() -> ! {
  let mut applications = [run_application::<Launcher>(), run_application::<Bar>()];

  while let Some(request) = pin!(run_ipc_server()).next().await {
    match request.kind() {
      Request::Version => request.reply(Response::version()),
      Request::Ping => request.reply(Response::Ping),

      _ => 'handler: {
        let mut request = request;
        for app in applications.iter_mut() {
          request = match app.try_send_request(request) {
            ControlFlow::Continue(request) => request,
            ControlFlow::Break(_) => break 'handler,
          }
        }

        request.reply(Err(
          "No applications were available for said request".into(),
        ));
      }
    }
  }

  eprintln!("Fatal error occured, quitting");
  process::exit(1)
}

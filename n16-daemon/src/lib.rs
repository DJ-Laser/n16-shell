use std::{ops::ControlFlow, pin::pin, process::ExitCode};

use bar::Bar;
use futures_lite::StreamExt;
use launcher::LauncherDaemon;
use n16_ipc::{Request, Response};

use crate::{application::run_application, ipc::run_ipc_server};

mod application;
mod bar;
mod ipc;
mod launcher;

pub async fn run_daemon() -> ExitCode {
  let mut applications = [
    run_application::<LauncherDaemon>(),
    run_application::<Bar>(),
  ];

  while let Some(request) = pin!(run_ipc_server()).next().await {
    match request.kind() {
      Request::Version => request.reply(Response::version()),
      Request::Ping => request.reply(Response::Ping),

      _ => 'handler: {
        let mut request = request;
        for app in &mut applications {
          request = match app.try_send_request(request) {
            ControlFlow::Continue(request) => request,
            ControlFlow::Break(()) => break 'handler,
          }
        }

        request.reply(Err(
          "No applications were available for said request".into(),
        ));
      }
    }
  }

  eprintln!("Fatal error occured, quitting");
  ExitCode::FAILURE
}

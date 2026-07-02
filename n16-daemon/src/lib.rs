use std::{marker::PhantomData, ops::ControlFlow, pin::pin, process};

use futures_lite::StreamExt;
use n16_bar::BarApplication;
use n16_core::application::{ApplicationRequest, N16Application};
use n16_ipc::{Request, Response};
use n16_launcher::LauncherApplication;

use crate::ipc::run_ipc_server;

mod ipc;

trait OpaqueApplication {
  fn run() -> Self
  where
    Self: Sized;

  fn try_send_request(
    &mut self,
    request: ApplicationRequest<n16_ipc::Request>,
  ) -> ControlFlow<(), ApplicationRequest<n16_ipc::Request>>;
}

struct WrappedApplication<A: N16Application> {
  request_tx: async_channel::Sender<ApplicationRequest<A::Request>>,
  _a: PhantomData<A>,
}

impl<A, R> OpaqueApplication for WrappedApplication<A>
where
  R: TryFrom<n16_ipc::Request, Error = n16_ipc::Request> + Send,
  A: N16Application<Request = R> + 'static + Send,
{
  fn run() -> Self {
    let (request_tx, request_rx) = async_channel::unbounded();
    tokio::spawn(A::run(request_rx));

    Self {
      request_tx,
      _a: PhantomData,
    }
  }

  fn try_send_request(
    &mut self,
    (request, reply_channel): ApplicationRequest<n16_ipc::Request>,
  ) -> ControlFlow<(), ApplicationRequest<n16_ipc::Request>> {
    match R::try_from(request) {
      Ok(request) => {
        let _ = self.request_tx.try_send((request, reply_channel));
        ControlFlow::Break(())
      }
      Err(request) => ControlFlow::Continue((request, reply_channel)),
    }
  }
}

pub async fn run_daemon() -> ! {
  let launcher: Box<dyn OpaqueApplication> =
    Box::new(WrappedApplication::<LauncherApplication>::run());

  let bar: Box<dyn OpaqueApplication> = Box::new(WrappedApplication::<BarApplication>::run());

  let mut applications = [launcher, bar];

  while let Some(mut request) = pin!(run_ipc_server()).next().await {
    match request.0 {
      Request::Version => {
        let _ = request.1.send(Response::version().reply_ok());
      }

      Request::Ping => {
        let _ = request.1.send(Response::Ping.reply_ok());
      }

      _ => {
        let mut request = request;
        for app in applications.iter_mut() {
          request = match app.try_send_request(request) {
            ControlFlow::Continue(request) => request,
            ControlFlow::Break(_) => break,
          }
        }
      }
    }
  }

  eprintln!("Fatal error occured, quitting");
  process::exit(1)
}

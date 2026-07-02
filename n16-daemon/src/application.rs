use std::{marker::PhantomData, ops::ControlFlow, thread};

use n16_core::application::{ApplicationRequest, N16Application};

pub trait OpaqueApplication {
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

impl<A> OpaqueApplication for WrappedApplication<A>
where
  A: N16Application + 'static + Send,
  A::Request: TryFrom<n16_ipc::Request, Error = n16_ipc::Request> + Send + 'static,
{
  fn run() -> Self {
    let (request_tx, request_rx) = async_channel::unbounded();
    thread::spawn(|| A::run(request_rx));

    Self {
      request_tx,
      _a: PhantomData,
    }
  }

  fn try_send_request(
    &mut self,
    request: ApplicationRequest<n16_ipc::Request>,
  ) -> ControlFlow<(), ApplicationRequest<n16_ipc::Request>> {
    match request.convert_kind() {
      Ok(request) => {
        let _ = self.request_tx.try_send(request);
        ControlFlow::Break(())
      }
      Err(request) => ControlFlow::Continue(request),
    }
  }
}

pub type DynApplication = Box<dyn OpaqueApplication>;

pub fn run_application<A>() -> DynApplication
where
  A: N16Application + 'static + Send,
  A::Request: TryFrom<n16_ipc::Request, Error = n16_ipc::Request> + Send + 'static,
{
  Box::new(WrappedApplication::<A>::run())
}

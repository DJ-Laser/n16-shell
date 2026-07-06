use std::{fmt::Debug, marker::PhantomData, ops::ControlFlow, thread};

#[derive(Debug, Clone)]
pub struct ApplicationRequest<R> {
  request: R,
  reply_tx: async_channel::Sender<n16_ipc::Reply>,
}

impl<R: Debug> ApplicationRequest<R> {
  pub fn new(request: R) -> (Self, async_channel::Receiver<n16_ipc::Reply>) {
    let (reply_tx, reply_rx) = async_channel::bounded(1);
    (Self { request, reply_tx }, reply_rx)
  }

  pub fn kind(&self) -> &R {
    &self.request
  }

  pub fn reply(self, reply: impl Into<n16_ipc::Reply>) {
    if let Err(error) = self.reply_tx.try_send(reply.into()) {
      eprintln!("Error handling request {:?}: {}", self.request, error)
    }
  }

  pub fn convert_kind<R2: TryFrom<R, Error = R>>(self) -> Result<ApplicationRequest<R2>, Self> {
    match R2::try_from(self.request) {
      Ok(new_request) => Ok(ApplicationRequest::<R2> {
        request: new_request,
        reply_tx: self.reply_tx,
      }),
      Err(request) => Err(Self {
        request,
        reply_tx: self.reply_tx,
      }),
    }
  }
}

pub type RequestChannel<R> = async_channel::Receiver<ApplicationRequest<R>>;

pub trait N16Application {
  type Request: TryFrom<n16_ipc::Request>;

  fn run(request_rx: RequestChannel<Self::Request>);
}

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

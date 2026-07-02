use std::fmt::Debug;

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

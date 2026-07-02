use std::fmt::Debug;

use async_trait::async_trait;
pub trait ShellMessage: Debug + Send + 'static {}
impl<T: Debug + Send + 'static> ShellMessage for T {}

pub type ApplicationRequest<R> = (R, async_oneshot::Sender<n16_ipc::Reply>);
pub type RequestChannel<R> = async_channel::Receiver<ApplicationRequest<R>>;

pub mod thread;

#[async_trait]
pub trait N16Application {
  type Request: TryFrom<n16_ipc::Request>;

  async fn run(request_channel: RequestChannel<Self::Request>);
}

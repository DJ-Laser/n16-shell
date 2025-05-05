use iced::{futures::channel::oneshot, Task};
use n16_ipc::{Reply, Request};

use crate::ShellMessage;

pub trait RequestHandler {
  type Request: TryFrom<Request>;
  type Message: ShellMessage;

  fn handle_request(
    &mut self,
    request: Self::Request,
    reply_channel: oneshot::Sender<Reply>,
  ) -> Task<Self::Message>;
}

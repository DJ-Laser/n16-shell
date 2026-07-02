use std::thread::{self};

pub struct Panicked;

pub struct IcedThread<RetVal> {
  ret_rx: async_oneshot::Receiver<RetVal>,
}

impl<RetVal> IcedThread<RetVal> {
  pub fn start<F, Message>(start_fn: F) -> (Self, async_channel::Sender<Message>)
  where
    F: (FnOnce(async_channel::Receiver<Message>) -> RetVal),
    F: Send + 'static,
    RetVal: Send + Sync + 'static,
    Message: Send + 'static,
  {
    let (message_tx, message_rx) = async_channel::unbounded();
    let (mut ret_tx, ret_rx) = async_oneshot::oneshot();

    thread::spawn(move || {
      let ret_val = start_fn(message_rx);
      let _ = ret_tx.send(ret_val);
    });

    (Self { ret_rx }, message_tx)
  }

  /// Will always return Ok(RetVal) unless the thread panics
  pub async fn await_return(self) -> Result<RetVal, Panicked> {
    self.ret_rx.await.map_err(|_| Panicked)
  }
}

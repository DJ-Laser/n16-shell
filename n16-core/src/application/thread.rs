use std::{
  sync::{Arc, Mutex},
  thread::{self},
};

use iced::futures::channel::{mpsc, oneshot};

#[derive(Debug)]
pub struct MessageStream<Message>(Arc<Mutex<Option<mpsc::Receiver<Message>>>>);

impl<Message> MessageStream<Message> {
  fn new(message_rx: mpsc::Receiver<Message>) -> Self {
    Self(Arc::new(Mutex::new(Some(message_rx))))
  }

  pub fn reciever(&self) -> Option<mpsc::Receiver<Message>> {
    self.0.lock().ok().map(|mut message_rx| message_rx.take())?
  }
}

impl<Message> Clone for MessageStream<Message> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

pub struct Panicked;

pub struct IcedThread<RetVal> {
  ret_rx: oneshot::Receiver<RetVal>,
}

impl<RetVal> IcedThread<RetVal> {
  pub fn start<F, Message>(start_fn: F) -> (Self, mpsc::Sender<Message>)
  where
    F: (FnOnce(MessageStream<Message>) -> RetVal),
    F: Send + 'static,
    RetVal: Send + 'static,
    Message: Send + 'static,
  {
    let (message_tx, message_rx) = mpsc::channel(5);
    let (ret_tx, ret_rx) = oneshot::channel();
    let message_rx = MessageStream::new(message_rx);

    thread::spawn(move || {
      let cloned_rx = message_rx.clone();
      let ret_val = start_fn(cloned_rx);
      let _ = ret_tx.send(ret_val);
    });

    (Self { ret_rx }, message_tx)
  }

  /// Will always return Ok(RetVal) unless the thread panics
  pub async fn await_return(self) -> Result<RetVal, Panicked> {
    self.ret_rx.await.map_err(|_| Panicked)
  }
}

use async_trait::async_trait;
use iced::Task;

use iced_layershell::settings::{LayerShellSettings, StartMode};
use n16_core::application::{N16Application, RequestChannel, thread::IcedThread};
use n16_ipc::bar::{Request, Response};

use crate::bar::{Bar, Message};

mod bar;

pub struct BarApplication {
  request_channel: RequestChannel<Request>,
  message_tx: async_channel::Sender<Message>,
}

#[async_trait]
impl N16Application for BarApplication {
  type Request = Request;

  async fn run(request_channel: RequestChannel<Self::Request>) {
    Self::new(request_channel).run().await;
  }
}

impl BarApplication {
  fn new(request_channel: RequestChannel<Request>) -> Self {
    let (_iced_thread, message_tx) = IcedThread::start(|message_rx| {
      iced_layershell::daemon(
        move || (Bar::new(), Task::stream(message_rx.clone())),
        "n16_bar",
        Bar::update,
        Bar::view,
      )
      .subscription(Bar::subscription)
      .layer_settings(LayerShellSettings {
        start_mode: StartMode::Background,
        ..Default::default()
      })
      .run()
    });

    Self {
      request_channel,
      message_tx,
    }
  }

  async fn run(&mut self) {
    while let Ok((request, mut reply_channel)) = self.request_channel.recv().await {
      let _ = self.message_tx.try_send(match request {
        Request::Show => {
          let _ = reply_channel.send(Response::handled().reply_ok());
          Message::ShowBar(true)
        }

        Request::Hide => {
          let _ = reply_channel.send(Response::handled().reply_ok());
          Message::ShowBar(false)
        }
      });
    }
  }
}

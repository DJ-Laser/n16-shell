use async_trait::async_trait;
use iced::{
  Task,
  futures::{SinkExt, StreamExt, channel::mpsc},
};
use iced_layershell::{
  actions::LayerShellCustomActionWithId, reexport::Anchor, settings::LayerShellSettings,
};
use listings::{Listing, Provider};
use n16_application::{N16Application, RequestChannel, thread::IcedThread};
use n16_ipc::{Response, launcher::Request};
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::gui::{Launcher, Message};

mod calculator;
mod component;
mod gui;
pub mod listings;
pub mod providers;

type Providers = Arc<Mutex<Vec<Box<dyn Provider>>>>;
type Listings = Vec<Box<dyn Listing>>;

impl TryInto<LayerShellCustomActionWithId> for Message {
  type Error = Self;

  fn try_into(self) -> Result<LayerShellCustomActionWithId, Self::Error> {
    Err(self)
  }
}

pub struct LauncherApplication {
  providers: Providers,
  listings: Listings,

  request_channel: RequestChannel<Request>,
  message_tx: Option<mpsc::Sender<Message>>,
}

#[async_trait]
impl N16Application for LauncherApplication {
  type Request = Request;

  async fn run(request_channel: RequestChannel<Self::Request>) {
    let mut this = Self::new(request_channel);
    this
      .add_provider(providers::ApplicationProvider::new())
      .await;
    this
      .add_provider(providers::PowerManagementProvider::new())
      .await;

    this.run().await;
  }
}

impl LauncherApplication {
  fn new(request_channel: RequestChannel<Request>) -> Self {
    Self {
      providers: Default::default(),
      listings: Vec::new(),
      request_channel,
      message_tx: None,
    }
  }

  async fn run(&mut self) {
    self.update_listings().await;

    while let Some((request, reply_channel)) = self.request_channel.next().await {
      match request {
        Request::Open => {
          let _ = reply_channel.send(Response::Handled.reply_ok());
          let _thread = self.open_launcher();
        }
        Request::Close => {
          let _ = reply_channel.send(Response::Handled.reply_ok());
          if let Some(message_tx) = &mut self.message_tx {
            let _ = message_tx.send(Message::Close).await;
          }
        }
      }
    }
  }

  fn open_launcher(&mut self) -> IcedThread<Result<(), iced_layershell::Error>> {
    let listings = self.listings.clone();

    let (iced_thread, message_tx) = IcedThread::start(move |message_stream| {
      iced_layershell::application(
        move || {
          (
            Launcher::new(listings.clone()),
            Task::batch([
              message_stream.reciever().map_or(Task::none(), Task::stream),
              Task::done(Message::FocusInput),
            ]),
          )
        },
        "n16_launcher",
        Launcher::update,
        Launcher::view,
      )
      .layer_settings(LayerShellSettings {
        size: Some((1000, 600)),
        anchor: Anchor::Top,
        margin: (200, 0, 0, 0),
        ..Default::default()
      })
      .subscription(Launcher::subscription)
      .run()
    });

    self.message_tx = Some(message_tx);
    iced_thread
  }

  async fn add_provider<P: Provider + 'static>(&mut self, provider: P) {
    let mut providers = self.providers.lock().await;
    providers.push(Box::new(provider));
  }

  async fn update_listings(&mut self) -> Listings {
    let mut providers = self.providers.lock().await;

    let listings = providers
      .iter_mut()
      .filter_map(|provider| provider.update_listings())
      .flatten();

    listings.collect()
  }
}

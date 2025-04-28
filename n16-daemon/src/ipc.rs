use std::io;
use std::path::Path;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

use iced::futures;

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::{Stream, StreamExt};

use iced::futures::SinkExt;
use iced::stream;
use n16_ipc::{Reply, Request};
use tokio_stream::wrappers::UnixListenerStream;

type Output = mpsc::Sender<(Request, oneshot::Sender<Reply>)>;

pub fn run_ipc_server() -> impl Stream<Item = (Request, oneshot::Sender<Reply>)> {
  stream::channel(100, async move |output| {
    let socket_path = Path::new(n16_ipc::socket_path());

    if socket_path.exists() {
      // Remove old socket file
      std::fs::remove_file(&socket_path).unwrap();
    }

    let listener = UnixListener::bind(&socket_path).unwrap();
    let listener = UnixListenerStream::new(listener);

    listener
      .for_each_concurrent(3, async |stream| {
        let Ok(stream) = stream else {
          return;
        };

        let mut output = output.clone();

        if let Err(err) = handle_stream(stream, &mut output).await {
          println!("Error creating stream: {:?}", err)
        }
      })
      .await
  })
}

async fn handle_stream(mut stream: UnixStream, output: &mut Output) -> io::Result<()> {
  let (read, mut write) = stream.split();
  let mut buf = String::new();

  BufReader::new(read).read_line(&mut buf).await?;

  let request = serde_json::from_str(&buf).map_err(|err| err.to_string());

  let reply = match request {
    Ok(request) => process_request(request, output)
      .await
      .unwrap_or(Err("Internal Error".to_string())),
    Err(err) => Err(err),
  };

  let mut buf = serde_json::to_vec(&reply)?;
  buf.push(b'\n');
  write.write_all(&buf).await?;

  Ok(())
}

async fn process_request(request: Request, output: &mut Output) -> Option<Reply> {
  let (sender, reciever) = oneshot::channel();
  output.send((request, sender)).await.ok()?;

  let reply = reciever.await.ok()?;
  Some(reply)
}

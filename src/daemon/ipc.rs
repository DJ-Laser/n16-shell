use std::io;

use async_net::unix::{UnixListener, UnixStream};

use iced::futures;
use iced::futures::io::BufReader;

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use futures::{Stream, StreamExt};

use iced::futures::SinkExt;
use iced::stream;
use n16_ipc::Response;
use n16_ipc::{Reply, Request};

type Output = mpsc::Sender<(Request, oneshot::Sender<Reply>)>;

pub fn run_ipc_server() -> impl Stream<Item = (Request, oneshot::Sender<Reply>)> {
  stream::channel(100, |output| async move {
    let socket_path = n16_ipc::get_socket_path().unwrap();

    if socket_path.exists() {
      // Remove old socket file
      std::fs::remove_file(&socket_path).unwrap();
    }

    let listener = UnixListener::bind(&socket_path).unwrap();

    listener
      .incoming()
      .for_each_concurrent(3, |stream| async {
        let Ok(stream) = stream else {
          return;
        };

        if let Err(err) = handle_stream(stream, output.clone()).await {
          println!("Error creating stream: {:?}", err)
        }
      })
      .await
  })
}

async fn handle_stream(stream: UnixStream, output: Output) -> io::Result<()> {
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

async fn process_request(request: Request, mut output: Output) -> Option<Reply> {
  let reply = match request {
    Request::Version => Response::version().reply_ok(),
    Request::Launcher(_) => {
      let (sender, reciever) = oneshot::channel();

      output.send((request, sender)).await.ok()?;

      reciever.await.ok()?
    }
  };

  Some(reply)
}

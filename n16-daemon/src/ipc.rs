use std::io;
use std::path::Path;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

use futures_lite::{Stream, StreamExt};

use n16_ipc::{Reply, Request};
use tokio_stream::wrappers::UnixListenerStream;

type Output = async_channel::Sender<(Request, async_oneshot::Sender<Reply>)>;

pub fn run_ipc_server() -> impl Stream<Item = (Request, async_oneshot::Sender<Reply>)> {
  let (output, reciever) = async_channel::unbounded();

  tokio::spawn(async move {
    let socket_path = Path::new(n16_ipc::socket_path());

    if socket_path.exists() {
      // Remove old socket file
      std::fs::remove_file(socket_path).unwrap();
    }

    let listener = UnixListener::bind(socket_path).unwrap();
    let listener = UnixListenerStream::new(listener);

    listener
      .for_each(|stream| {
        let Ok(stream) = stream else {
          return;
        };

        let mut output = output.clone();

        tokio::spawn(async move {
          if let Err(err) = handle_stream(stream, &mut output).await {
            println!("Error creating stream: {:?}", err)
          }
        });
      })
      .await;
  });

  reciever
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
  let (sender, reciever) = async_oneshot::oneshot();
  output.send((request, sender)).await.ok()?;

  let reply = reciever.await.ok()?;
  Some(reply)
}

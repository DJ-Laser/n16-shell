use std::{
  io::{self, BufRead, BufReader, Write},
  net::Shutdown,
  os::unix::net::UnixStream,
};

use n16_ipc::{Reply, Request};

pub fn send_request(request: Request) -> io::Result<Reply> {
  let mut stream = UnixStream::connect(n16_ipc::socket_path())?;

  let mut buf = serde_json::to_vec(&request)?;
  buf.push(b'\n');

  stream.write_all(&buf)?;

  let mut buf = String::new();
  BufReader::new(stream.try_clone()?).read_line(&mut buf)?;

  let reply = serde_json::from_str(&buf)
    .map_err(|err| err.to_string())
    .and_then(|op| op);

  stream.shutdown(Shutdown::Both)?;

  Ok(reply)
}

//! Ipc helpers for interfacting with the n16 daemon
//!
//! Inspired by https://crates.io/crates/niri-ipc
//!
//! Communication is done via unix socket. Requests and responses are both sent as json.

use std::{
  env::{self},
  path::PathBuf,
};

pub use api::*;

mod api;

pub const VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");

pub fn get_socket_path() -> Result<PathBuf, &'static str> {
  let runtime_path =
    env::var("XDG_RUNTIME_DIR").map_err(|_| "`XDG_RUNTIME_DIR` must be set and valid unicode")?;
  let mut runtime_path = PathBuf::from(runtime_path);
  runtime_path.push("n16-shell.sock");

  Ok(runtime_path)
}

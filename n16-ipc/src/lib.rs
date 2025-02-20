//! Ipc helpers for interfacting with the n16 daemon
//!
//! Inspired by https://crates.io/crates/niri-ipc
//!
//! Communication is done via unix socket. Requests and responses are both sent as json.

use std::{
  env::{self},
  path::PathBuf,
};

use git_version::git_version;

pub use api::*;

mod api;

pub fn version() -> String {
  const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
  const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
  const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

  let commit = git_version!(fallback = "unknown commit");

  if PATCH == "0" {
    format!("{MAJOR}.{MINOR:0>2} ({commit})")
  } else {
    format!("{MAJOR}.{MINOR:0>2}.{PATCH} ({commit})")
  }
}

pub fn get_socket_path() -> Result<PathBuf, &'static str> {
  let runtime_path =
    env::var("XDG_RUNTIME_DIR").map_err(|_| "`XDG_RUNTIME_DIR` must be set and valid unicode")?;
  let mut runtime_path = PathBuf::from(runtime_path);
  runtime_path.push("n16-shell.sock");

  Ok(runtime_path)
}

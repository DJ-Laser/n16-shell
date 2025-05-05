//! Ipc helpers for interfacting with the n16 daemon
//!
//! Inspired by https://crates.io/crates/niri-ipc
//!
//! Communication is done via unix socket. Requests and responses are both sent as json.

use std::{
  env::{self},
  sync::OnceLock,
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

fn read_socket_path() -> String {
  let runtime_path = env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| {
    let uid = rustix::process::getuid();
    format!("/run/user/{}", uid.as_raw())
  });

  let wayland_socket = std::env::var("WAYLAND_DISPLAY");
  let display = match wayland_socket.as_ref() {
    // If wayland_socket is a path, only use the last component
    Ok(wayland_socket) => wayland_socket.rsplit('/').next().unwrap(),

    Err(_) => {
      eprintln!("WARNING: WAYLAND_DISPLAY variable not set. Defaulting to wayland-0");
      "wayland-0.sock"
    }
  };

  format!("{runtime_path}/n16-shell-{display}.sock")
}

pub fn socket_path() -> &'static str {
  static PATH: OnceLock<String> = OnceLock::new();
  PATH.get_or_init(read_socket_path)
}

pub fn is_daemon_running() {}

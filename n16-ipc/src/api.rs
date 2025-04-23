use serde::{Deserialize, Serialize};

pub mod bar;
pub mod launcher;

/// Request sent to the n16 daemon
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Request {
  Version,
  Ping,

  Launcher(launcher::Request),
  Bar(bar::Request),
}

impl Request {
  /// Serialize this `Request` to json
  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }
}

/// Successful response from the n16 daemon
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Response {
  /// A request that does not need a response was handled successfully
  Handled,
  /// The `n16_ipc::VERSION_STRING` of the daemon.
  Version(String),
  /// Response to `Ping` requests
  Ping,

  Launcher(launcher::Response),
  Bar(bar::Response),
}

impl Response {
  /// Serialize this `Response` to json
  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

  /// Convert this `Response` to Reply::Ok()
  pub fn reply_ok(self) -> Reply {
    Reply::Ok(self)
  }

  /// Creates a `Response::Version()` containing `niri_ipc::VERSION_STRING`
  pub fn version() -> Self {
    Self::Version(crate::version())
  }
}

/// Reply from the n16 daemon
///
/// Every request gets one reply.
///
/// * If an error had occurred, it will be an `Reply::Err`.
/// * If the request does not need any particular response, it will be
///   `Reply::Ok(Response::Handled)`. Kind of like an `Ok(())`.
pub type Reply = Result<Response, String>;

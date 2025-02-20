use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Request {
  Open,
  Close,
}

impl From<Request> for super::Request {
  fn from(value: Request) -> Self {
    super::Request::Launcher(value)
  }
}

impl TryFrom<super::Request> for Request {
  type Error = super::Request;

  fn try_from(value: super::Request) -> Result<Self, Self::Error> {
    match value {
      super::Request::Launcher(launcher_value) => Ok(launcher_value),
      _ => Err(value),
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Response {}

impl Response {
  pub fn handled() -> super::Response {
    super::Response::Handled
  }
}

impl From<Response> for super::Response {
  fn from(value: Response) -> Self {
    super::Response::Launcher(value)
  }
}

impl TryFrom<super::Response> for Response {
  type Error = super::Response;

  fn try_from(value: super::Response) -> Result<Self, Self::Error> {
    match value {
      super::Response::Launcher(launcher_value) => Ok(launcher_value),
      _ => Err(value),
    }
  }
}

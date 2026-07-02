use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Request {
  Show,
  Hide,
}

impl From<Request> for super::Request {
  fn from(value: Request) -> Self {
    super::Request::Bar(value)
  }
}

impl TryFrom<super::Request> for Request {
  type Error = super::Request;

  fn try_from(value: super::Request) -> Result<Self, Self::Error> {
    match value {
      super::Request::Bar(bar_value) => Ok(bar_value),
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
    super::Response::Bar(value)
  }
}

impl TryFrom<super::Response> for Response {
  type Error = super::Response;

  fn try_from(value: super::Response) -> Result<Self, Self::Error> {
    match value {
      super::Response::Bar(bar_value) => Ok(bar_value),
      _ => Err(value),
    }
  }
}

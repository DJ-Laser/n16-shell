use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Request {
  Open,
  Close,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Response {}

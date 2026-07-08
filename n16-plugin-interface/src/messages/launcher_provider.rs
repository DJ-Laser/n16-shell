use serde::{Deserialize, Serialize};

use crate::config::Config;

/// Message specific to launcher provider instances.
#[derive(Serialize, Deserialize)]
pub struct LauncherProviderMessageWithId {
  pub instance_id: u64,
  pub message: LauncherProviderMessage,
}

/// See [`LauncherProviderMessageWithId`]
#[derive(Serialize, Deserialize)]
pub enum LauncherProviderMessage {
  /// See [`LauncherProviderInit`]
  Init(LauncherProviderInit),
}

/// Initialize a new launcher provider.\
/// The new provider must be assigned the `instance_id` recieved along with this message
///
/// Acceptable responses: `Response::Success`, `Response::Failure`.
#[derive(Serialize, Deserialize)]
pub struct LauncherProviderInit {
  /// Id of the launcher provider to initialize as defined in `plugin.kdl`
  pub id: String,
  /// Config for launcher provider with id `id` as defined in `plugin.kdl`
  pub config: Config,
}

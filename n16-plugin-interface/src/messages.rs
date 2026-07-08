use serde::{Deserialize, Serialize};

use crate::{config::Config, messages::launcher_provider::LauncherProviderMessage};

pub mod launcher_provider;

/// A message from the shell to a plugin.\
/// Messages to the plugin all have a unique `message_id`.
/// For every message recieved, the plugin must eventually send a response.
/// Multiple messages may be sent before a response, so responses must include the relevant `message_id` they are responding to.
/// The acceptable responses to each message are listed on the docs of their respective structs.
///
/// Some messages may invalidate prior messages. In this case the prior messages do not need to be responded to,
/// and any response for that `message_id` will be ignored.
/// More information can be found in docs of the relavant message structs.
#[derive(Serialize, Deserialize)]
pub struct MessageWithId {
  /// The content of the message.
  pub message: Message,
  /// The unique id of this message.
  pub message_id: u64,
}

/// See [`MessageWithId`]
#[derive(Serialize, Deserialize)]
pub enum Message {
  /// See [`PluginInit`]
  PluginInit(PluginInit),
  /// See [`LauncherProviderMessage`]
  LauncherProvider(LauncherProviderMessage),
}

/// Initialize the plugin with the provided config.
/// Sent once on plugin startup.
///
/// Acceptable responses: `Response::Success`, `Response::Failure`.
#[derive(Serialize, Deserialize)]
pub struct PluginInit {
  /// Plugin global config as defined in `plugin.kdl`
  pub plugin_config: Config,
}

/// Request the plugin to gracefully shut down.
/// This message invalidates all previous messages.
/// Sent once when requesting plugin shutdown.
///
/// Acceptable responses: `Response::Success`, `Response::Failure`.
#[derive(Serialize, Deserialize)]
pub struct PluginShutdown;

/// Response from a plugin to the shell
#[derive(Serialize, Deserialize)]
pub struct ResponseWithId {
  /// The content of the response.
  pub message: Response,
  /// The unique id of the message being responded to.
  pub message_id: u64,
}

/// See [`ResponseWithId`]
#[derive(Serialize, Deserialize)]
pub enum Response {
  /// The requested action was performed successfully.
  Success,
  /// The requested action failed.
  Faulure { reason: String },
}

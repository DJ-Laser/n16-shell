use std::process::ExitCode;

use n16_daemon::run_daemon;

#[tokio::main]
async fn main() -> ExitCode {
  run_daemon().await
}

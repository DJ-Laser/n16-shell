use n16_daemon::run_daemon;

#[tokio::main]
pub async fn main() {
  run_daemon().await
}

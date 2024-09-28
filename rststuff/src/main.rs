use basic_echo_server::basic_echo_server;

use std::sync::Arc;
use tokio::sync::Notify;

#[tokio::main]
async fn main() {
    // Spawn the echo server in a separate task
    let notify: Arc<Notify> = Arc::new(Notify::new());

    let notify_clone = Arc::clone(&notify);
    let _ = tokio::spawn(async move {
        basic_echo_server(notify_clone).await;
    });
}

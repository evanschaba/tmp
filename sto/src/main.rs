use lib::basic::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let addr = "127.0.0.1:8080";
    let db = Database::new(Some(DB_FILE))?;
    let server = Server::new(addr, db).await?;

    println!("Server running at {}", addr);

    // Handle shutdown signals
    let server_clone = server.clone();
    ctrlc::set_handler(move || {
        println!("Received shutdown signal");
        server_clone.shutdown();
    })?;

    server.run().await?;
    Ok(())
}

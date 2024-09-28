use log::{error, info};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Notify,
    time::{self, Duration},
};

//note: kill -9 $(lsof -t -i:8081); RUST_BACKTRACE=1 cargo test --lib

/// Basic Echo server implementation
pub async fn basic_echo_server(notify: Arc<Notify>) {
    let addr = "127.0.0.1:8081";

    match TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Server running on {}", addr);
            notify.notify_one(); // Notify that server is ready

            loop {
                match listener.accept().await {
                    Ok((mut tcp_stream, socket_addr)) => {
                        info!("Accepted connection from {}", socket_addr);
                        tokio::spawn(async move {
                            let result = handle_client(&mut tcp_stream).await;
                            if let Err(e) = result {
                                error!("Error handling client: {:?}", e);
                            }
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept connection: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to bind to {}: {:?}", addr, e);
        }
    }
}

/// Handles client communication by echoing back received data
async fn handle_client(tcp_stream: &mut tokio::net::TcpStream) -> Result<(), std::io::Error> {
    let mut buf = vec![0u8; 1024]; // 1 KB buffer

    loop {
        // Set a read timeout
        let read_result = time::timeout(Duration::from_secs(5), tcp_stream.read(&mut buf)).await;

        match read_result {
            // Successfully read some bytes and echo them back
            Ok(Ok(bytes_read)) if bytes_read > 0 => {
                tcp_stream.write_all(&buf[..bytes_read]).await?;
            }
            // If 1 or more bytes are read, handle it
            Ok(Ok(1_usize..)) => {
                info!("Read some data from client.");
                tcp_stream.write_all(&buf).await?;
            }
            // Connection closed by client
            Ok(Ok(0)) => {
                info!("Connection closed by client.");
                break;
            }
            // Error while reading from stream
            Ok(Err(e)) => {
                error!("Failed to read from stream: {:?}", e);
                return Err(e);
            }
            // Operation timed out
            Err(_) => {
                error!("Read operation timed out");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Read operation timed out",
                ));
            }
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::{net::TcpStream, sync::Notify};

   async fn wait_for_server(notify: Arc<Notify>) {
    println!("Waiting for server to start...");
    if let Ok(_) = time::timeout(Duration::from_secs(20), notify.notified()).await {
        println!("Server is ready!");
    } else {
        panic!("Server failed to start in time.");
    }
}

    #[tokio::test]
    async fn test_echo_server() {
        let addr = "127.0.0.1:8081";
        let notify = Arc::new(Notify::new());

        // Spawn the echo server in a separate task
        let notify_clone = Arc::clone(&notify);
        let server_handle = tokio::spawn(async move {
            basic_echo_server(notify_clone).await;
        });

        // Wait for the server to be ready
        wait_for_server(Arc::clone(&notify)).await;

        // Connect to the echo server
        let mut client = TcpStream::connect(addr).await.unwrap();
        let msg = b"Hello, world!";
        client.write_all(msg).await.unwrap();

        let mut buf = vec![0u8; 1024];
        let bytes_read = client.read(&mut buf).await.unwrap();
        assert_eq!(&buf[..bytes_read], msg);

        // Shutdown server
        server_handle.abort();
    }
    #[tokio::test]
    async fn test_timeout() {
        let addr = "127.0.0.1:8081";
        let notify = Arc::new(Notify::new());
    
        // Spawn the echo server in a separate task
        let notify_clone = Arc::clone(&notify);
        let server_handle = tokio::spawn(async move {
            basic_echo_server(notify_clone).await;
        });
    
        // Wait for the server to be ready
        wait_for_server(Arc::clone(&notify)).await;
    
        // Introduce a small delay to ensure that the server is ready for connections
        tokio::time::sleep(Duration::from_millis(100)).await;
    
        // Simulate a client that takes too long to respond
        let mut client = TcpStream::connect(addr).await.unwrap();
        let result = time::timeout(Duration::from_secs(1), async {
            let mut buf = vec![0u8; 1024];
            client.read(&mut buf).await
        })
        .await;
    
        assert!(result.is_err(), "The read operation should timeout");
    
        // Shutdown server
        server_handle.abort();
    }
    
}

// use lib::basic::*;
// use std::time::Duration as StdDuration;
// use tokio::net::UdpSocket;
// use tokio::time::{sleep, timeout, Duration};

// const TEST_TIMEOUT: Duration = Duration::from_secs(5);
// const SERVER_STARTUP_DELAY: Duration = Duration::from_millis(100);

// #[tokio::test]
// async fn test_udp_server() -> Result<()> {
//     let addr = "127.0.0.1:8081";
//     let db = Database::new(Some(":memory:"))?;
//     let server = Server::new(addr, db).await?;

//     // Start server with timeout
//     let server_clone = server.clone();
//     let server_handle = tokio::spawn(async move {
//         server_clone.run_with_timeout(TEST_TIMEOUT).await.unwrap();
//     });

//     sleep(SERVER_STARTUP_DELAY).await;

//     let client = UdpSocket::bind("127.0.0.1:0").await?;
//     client.set_read_timeout(Some(StdDuration::from_secs(1)))?;

//     // Test suite
//     let tests = vec![
//         (
//             r#"CREATE list_key {"List":[{"name":"Alice"},{"name":"Bob"}]}"#,
//             "Created successfully",
//         ),
//         (
//             "READ list_key",
//             r#"{"List":[{"name":"Alice"},{"name":"Bob"}]}"#,
//         ),
//     ];

//     for (request, expected_response) in tests {
//         client.send_to(request.as_bytes(), addr).await?;

//         let mut buf = vec![0; 1024];
//         let result = timeout(StdDuration::from_secs(1), client.recv_from(&mut buf)).await;
//         match result {
//             Ok(Ok((len, _))) => {
//                 assert_eq!(
//                     String::from_utf8_lossy(&buf[..len]),
//                     expected_response,
//                     "Failed on request: {}",
//                     request
//                 );
//             }
//             Ok(Err(e)) => {
//                 panic!("Failed to receive data: {}", e);
//             }
//             Err(_) => {
//                 panic!("Request timed out for: {}", request);
//             }
//         }
//     }

//     // Cleanup
//     server.shutdown();
//     server_handle.await.map_err(|e| DatabaseError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

//     Ok(())
// }
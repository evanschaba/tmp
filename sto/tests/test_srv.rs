use lib::basic::*;
use tokio::net::UdpSocket;
// use std::time::Duration as StdDuration;
use tokio::time::{sleep, /*timeout,*/ Duration};

const TEST_ADDR: &str = "127.0.0.1:8081";
const SERVER_STARTUP_DELAY: Duration = Duration::from_millis(100);

async fn setup_test_server() -> Server {
    let db = Database::new(Some(":memory:")).unwrap();
    let server = Server::new(TEST_ADDR, db).await.unwrap();
    let server_clone = server.clone();
    tokio::spawn(async move {
        server_clone
            .run_with_timeout(Duration::from_secs(10))
            .await
            .unwrap();
    });
    sleep(SERVER_STARTUP_DELAY).await;
    server
}

#[tokio::test]
async fn test_create_and_read() {
    let _server = setup_test_server().await;
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();

    client
        .send_to(
            b"CREATE test_key {\"Single\":{\"name\":\"Alice\"}}",
            TEST_ADDR,
        )
        .await
        .unwrap();

    let mut buf = [0; 1024];
    let len = client.recv(&mut buf).await.unwrap();
    assert_eq!(
        std::str::from_utf8(&buf[..len]).unwrap(),
        "Created successfully"
    );

    client.send_to(b"READ test_key", TEST_ADDR).await.unwrap();
    let len = client.recv(&mut buf).await.unwrap();
    assert_eq!(
        std::str::from_utf8(&buf[..len]).unwrap(),
        "{\"Single\":{\"name\":\"Alice\"}}"
    );
}

#[tokio::test]
async fn test_update_and_delete() { // TODO:   left: "Unknown command" && right: "Deleted successfully"
    let _server = setup_test_server().await;
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();

    client
        .send_to(
            b"CREATE test_key {\"Single\":{\"name\":\"Alice\"}}",
            TEST_ADDR,
        )
        .await
        .unwrap();
    client.recv(&mut [0; 1024]).await.unwrap();

    client
        .send_to(
            b"UPDATE test_key {\"Single\":{\"name\":\"Bob\"}}",
            TEST_ADDR,
        )
        .await
        .unwrap();
    let mut buf = [0; 1024];
    let len = client.recv(&mut buf).await.unwrap();
    assert_eq!(
        std::str::from_utf8(&buf[..len]).unwrap(),
        "Updated successfully"
    );

    client.send_to(b"READ test_key", TEST_ADDR).await.unwrap();
    let len = client.recv(&mut buf).await.unwrap();
    assert_eq!(
        std::str::from_utf8(&buf[..len]).unwrap(),
        "{\"Single\":{\"name\":\"Bob\"}}"
    );

    client.send_to(b"DELETE test_key", TEST_ADDR).await.unwrap();
    let len = client.recv(&mut buf).await.unwrap();
    assert_eq!(
        std::str::from_utf8(&buf[..len]).unwrap(),
        "Deleted successfully"
    );
}

#[tokio::test]
async fn test_invalid_commands() {
    let _server = setup_test_server().await;
    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();

    client.send_to(b"INVALID command", TEST_ADDR).await.unwrap();
    let mut buf = [0; 1024];
    let len = client.recv(&mut buf).await.unwrap();
    assert_eq!(std::str::from_utf8(&buf[..len]).unwrap(), "Unknown command");

    client
        .send_to(
            b"CREATE malformed_key {\"Single\":{\"name\":\"Alice\"",
            TEST_ADDR,
        )
        .await
        .unwrap();
    let len = client.recv(&mut buf).await.unwrap();
    assert_eq!(
        std::str::from_utf8(&buf[..len]).unwrap(),
        "Invalid data format"
    );
}

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

//     // Define test cases
//     let tests = vec![
//         (
//             r#"CREATE list_key {"List":[{"name":"Alice"},{"name":"Bob"}]}"#,
//             "Created successfully",
//         ),
//         (
//             "READ list_key",
//             r#"{"List":[{"name":"Alice"},{"name":"Bob"}]}"#,
//         ),
//         (
//             r#"UPDATE list_key {"List":[{"name":"Alice"},{"name":"Charlie"}]}"#,
//             "Updated successfully",
//         ),
//         (
//             "READ list_key",
//             r#"{"List":[{"name":"Alice"},{"name":"Charlie"}]}"#,
//         ),
//         (
//             "CREATE single_key {\"Single\":{\"name\":\"Eve\"}}",
//             "Created successfully",
//         ),
//         (
//             "READ single_key",
//             r#"{"Single":{"name":"Eve"}}"#,
//         ),
//         (
//             "DELETE list_key",
//             "Deleted successfully",
//         ),
//         (
//             "READ list_key",
//             "Key not found",
//         ),
//         (
//             "INVALID_COMMAND key",
//             "Unknown command",
//         ),
//         (
//             r#"CREATE invalid_json {"List":[{"name":"Alice""#, // Malformed JSON
//             "Invalid data format",
//         ),
//         (
//             "READ missing_key",
//             "Key not found",
//         ),
//     ];

//     // Run test cases
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
//             Ok(Err(e)) => panic!("Failed to receive data: {}", e),
//             Err(_) => panic!("Request timed out for: {}", request),
//         }
//     }

//     // Cleanup
//     server.shutdown();
//     server_handle
//         .await
//         .map_err(|e| DatabaseError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

//     Ok(())
// }

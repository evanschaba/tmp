use lib::basic::*;
use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

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
async fn test_update_and_delete() {
    // TODO: ✅
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

    client.send_to(b"READ test_key", TEST_ADDR).await.unwrap();
    let len = client.recv(&mut buf).await.unwrap();
    assert_eq!(std::str::from_utf8(&buf[..len]).unwrap(), "Key not found");
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

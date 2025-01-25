use lib::basic::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

/// BAAH !! IT HANGS !! IT ALL HANGS 😫 😫 😫 😫 😫

const TEST_TIMEOUT: Duration = Duration::from_secs(2);
const OPERATION_TIMEOUT: Duration = Duration::from_millis(150);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Animal {
    name: String,
    species: String,
}

// async fn setup_test_server<T>() -> (Server<T>, SocketAddr, UdpSocket)
// where
//     T: Serialize + for<'a> Deserialize<'a> + PartialEq + Clone + Send + Sync + 'static,
// {
//     let port = get_next_available_port().await;
//     let addr = format!("127.0.0.1:{}", port);
//     let db = Database::<T>::new(Some(":memory:")).unwrap();
//     let server = Server::new(&addr, db).await.unwrap();
//     let server_addr = server.get_addr();

//     let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();

//     // Spawn server in the background with a timeout
//     tokio::spawn({
//         let server_clone = server.clone();
//         async move {
//             if let Err(e) = timeout(TEST_TIMEOUT, server_clone.run()).await {
//                 eprintln!("Server run timeout: {}", e);
//                 server_clone.shutdown();
//             }
//         }
//     });

//     // Give the server a moment to start
//     tokio::time::sleep(OPERATION_TIMEOUT).await;

//     (server, server_addr, client)
// }

async fn send_and_verify(
    client: &UdpSocket,
    addr: &SocketAddr,
    message: &str,
    expected_response: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    client.send_to(message.as_bytes(), addr).await?;

    let mut buf = [0; 1024];
    let len = timeout(Duration::from_secs(1), client.recv(&mut buf)).await??;
    let response = std::str::from_utf8(&buf[..len])?;
    assert_eq!(response, expected_response);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_create_human() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (server, addr, client) = setup_test_server::<Human>().await;

    // Test CREATE
    send_and_verify(
        &client,
        &addr,
        "CREATE test_human {\"Single\":{\"name\":\"Alice\"}}",
        "Created successfully",
    )
    .await?;

    server.shutdown();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_read_human() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (server, addr, client) = setup_test_server::<Human>().await;

    // First create a human
    send_and_verify(
        &client,
        &addr,
        "CREATE test_human {\"Single\":{\"name\":\"Alice\"}}",
        "Created successfully",
    )
    .await?;

    // Then read
    send_and_verify(
        &client,
        &addr,
        "READ test_human",
        "{\"Single\":{\"name\":\"Alice\"}}",
    )
    .await?;

    server.shutdown();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_create_animal() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (server, addr, client) = setup_test_server::<Animal>().await;

    // Test CREATE
    send_and_verify(
        &client,
        &addr,
        "CREATE test_animal {\"Single\":{\"name\":\"Buddy\",\"species\":\"Dog\"}}",
        "Created successfully",
    )
    .await?;

    server.shutdown();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_read_animal() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (server, addr, client) = setup_test_server::<Animal>().await;

    // First create an animal
    send_and_verify(
        &client,
        &addr,
        "CREATE test_animal {\"Single\":{\"name\":\"Buddy\",\"species\":\"Dog\"}}",
        "Created successfully",
    )
    .await?;

    // Then read
    send_and_verify(
        &client,
        &addr,
        "READ test_animal",
        "{\"Single\":{\"name\":\"Buddy\",\"species\":\"Dog\"}}",
    )
    .await?;

    server.shutdown();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_create_human_list() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (server, addr, client) = setup_test_server::<Human>().await;

    // Create initial list
    send_and_verify(
        &client,
        &addr,
        "CREATE humans_list {\"List\":[{\"name\":\"Alice\"}]}",
        "Created successfully",
    )
    .await?;

    server.shutdown();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_append_human_list() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (server, addr, client) = setup_test_server::<Human>().await;

    // First create a list
    send_and_verify(
        &client,
        &addr,
        "CREATE humans_list {\"List\":[{\"name\":\"Alice\"}]}",
        "Created successfully",
    )
    .await?;

    // Test append
    send_and_verify(
        &client,
        &addr,
        "APPEND humans_list [{\"name\":\"Bob\"}]",
        "Appended successfully",
    )
    .await?;

    server.shutdown();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_read_human_list_after_append() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    let (server, addr, client) = setup_test_server::<Human>().await;

    // Create a list
    send_and_verify(
        &client,
        &addr,
        "CREATE humans_list {\"List\":[{\"name\":\"Alice\"}]}",
        "Created successfully",
    )
    .await?;

    // Append a human
    send_and_verify(
        &client,
        &addr,
        "APPEND humans_list [{\"name\":\"Bob\"}]",
        "Appended successfully",
    )
    .await?;

    // Verify list state
    send_and_verify(
        &client,
        &addr,
        "READ humans_list",
        "{\"List\":[{\"name\":\"Alice\"},{\"name\":\"Bob\"}]}",
    )
    .await?;

    server.shutdown();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_remove_human_list() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let (server, addr, client) = setup_test_server::<Human>().await;

    // Create a list
    send_and_verify(
        &client,
        &addr,
        "CREATE humans_list {\"List\":[{\"name\":\"Alice\"},{\"name\":\"Bob\"}]}",
        "Created successfully",
    )
    .await?;

    // Test remove
    send_and_verify(
        &client,
        &addr,
        "REMOVE humans_list {\"name\":\"Bob\"}",
        "Removed successfully",
    )
    .await?;

    server.shutdown();
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_read_human_list_after_remove() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    let (server, addr, client) = setup_test_server::<Human>().await;

    // Create a list
    send_and_verify(
        &client,
        &addr,
        "CREATE humans_list {\"List\":[{\"name\":\"Alice\"},{\"name\":\"Bob\"}]}",
        "Created successfully",
    )
    .await?;

    // Remove Bob
    send_and_verify(
        &client,
        &addr,
        "REMOVE humans_list {\"name\":\"Bob\"}",
        "Removed successfully",
    )
    .await?;

    // Verify final state
    send_and_verify(
        &client,
        &addr,
        "READ humans_list",
        "{\"List\":[{\"name\":\"Alice\"}]}",
    )
    .await?;

    server.shutdown();
    Ok(())
}

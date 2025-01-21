use lib::basic::*;
use std::time::Duration as StdDuration;
use tokio::net::UdpSocket;
use tokio::time::{sleep, timeout, Duration};

const TEST_TIMEOUT: Duration = Duration::from_secs(5);
const SERVER_STARTUP_DELAY: Duration = Duration::from_millis(100);

#[tokio::test]
async fn test_udp_server() -> Result<()> {
    let addr = "127.0.0.1:8081";
    let db = Database::new(Some(":memory:"))?;
    let server = Server::new(addr, db).await?;
    
    // Start server with timeout
    let server_clone = server.clone();
    let server_handle = tokio::spawn(async move {
        server_clone.run_with_timeout(TEST_TIMEOUT).await.unwrap();
    });

    sleep(SERVER_STARTUP_DELAY).await;

    let client = UdpSocket::bind("127.0.0.1:0").await?;
    client.set_read_timeout(Some(StdDuration::from_secs(1)))?;

    // Test suite
    let tests = vec![
        (
            r#"CREATE list_key {"List":[{"name":"Alice"},{"name":"Bob"}]}"#,
            "Created successfully",
        ),
        (
            "READ list_key",
            r#"{"List":[{"name":"Alice"},{"name":"Bob"}]}"#,
        ),
    ];

    for (request, expected_response) in tests {
        client.send_to(request.as_bytes(), addr).await?;
        
        let mut buf = vec![0; 1024];
        let result = timeout(StdDuration::from_secs(1), client.recv_from(&mut buf)).await;
        match result {
            Ok(Ok((len, _))) => {
                assert_eq!(
                    String::from_utf8_lossy(&buf[..len]),
                    expected_response,
                    "Failed on request: {}",
                    request
                );
            }
            Ok(Err(e)) => {
                panic!("Failed to receive data: {}", e);
            }
            Err(_) => {
                panic!("Request timed out for: {}", request);
            }
        }
    }

    // Cleanup
    server.shutdown();
    server_handle.await.map_err(|e| DatabaseError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    
    Ok(())
}

#[tokio::test]
async fn test_database_operations() -> Result<()> {
    let db = Database::new(Some(":memory:"))?;

    // Test CREATE and READ
    let key = "test_key".to_string();
    let resource = Resource::List(vec![
        Human {
            name: "Alice".to_string(),
        },
        Human {
            name: "Bob".to_string(),
        },
    ]);

    db.create(key.clone(), resource.clone())?;
    assert_eq!(db.read(&key)?, Some(resource.clone()));

    // Test APPEND
    let new_items = vec![Human {
        name: "Charlie".to_string(),
    }];
    db.append_to_list(&key, new_items)?;

    if let Some(Resource::List(list)) = db.read(&key)? {
        assert_eq!(list.len(), 3);
        assert_eq!(list[2].name, "Charlie");
    } else {
        panic!("Expected list resource");
    }

    // Test REMOVE
    db.remove_from_list(&key, "Bob")?;

    if let Some(Resource::List(list)) = db.read(&key)? {
        assert_eq!(list.len(), 2);
        assert!(!list.iter().any(|h| h.name == "Bob"));
    } else {
        panic!("Expected list resource");
    }

    // Test DELETE
    db.delete(&key)?;
    assert_eq!(db.read(&key)?, None);

    Ok(())
}

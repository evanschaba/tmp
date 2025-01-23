use log::{error, info};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use thiserror::Error;
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, mpsc};
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time::timeout;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Invalid resource type: {0}")]
    InvalidResourceType(String),
    #[error("Lock error")]
    LockError,
    #[error("Ctrl-C error: {0}")]
    CtrlC(#[from] ctrlc::Error),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Human {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Resource<T> {
    Single(T),
    List(Vec<T>),
}

pub type Db<T> = HashMap<String, Resource<T>>;

pub const DB_FILE: &str = "db.json";

#[derive(Debug)]
pub struct Database<T> {
    data: Arc<Mutex<Db<T>>>,
    file_path: String,
}

#[derive(Clone, Debug)]
pub struct Server<T> {
    socket: Arc<UdpSocket>,
    db: Arc<Database<T>>,
    shutdown: Arc<broadcast::Sender<()>>,
    addr: SocketAddr,
    handler: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl<T> Database<T>
where
    T: Serialize + for<'a> Deserialize<'a> + PartialEq + Clone,
{
    pub fn new(file_path: Option<&str>) -> Result<Self> {
        let path = file_path.unwrap_or(DB_FILE).to_string();

        let data = if std::path::Path::new(&path).exists() {
            let mut file = File::open(&path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            serde_json::from_str(&content).unwrap_or_else(|_| HashMap::new())
        } else {
            HashMap::new()
        };

        Ok(Database {
            data: Arc::new(Mutex::new(data)), // Fixed: Wrapped in Arc
            file_path: path,
        })
    }

    fn save_to_disk(&self) -> Result<()> {
        if self.file_path.is_empty() {
            return Ok(());
        }

        let data = self.data.lock().map_err(|_| DatabaseError::LockError)?;
        let json = serde_json::to_string(&*data)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn create(&self, key: String, resource: Resource<T>) -> Result<()> {
        let mut data = self.data.lock().map_err(|_| DatabaseError::LockError)?;
        data.insert(key, resource);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn read(&self, key: &str) -> Result<Option<Resource<T>>> {
        let data = self.data.lock().map_err(|_| DatabaseError::LockError)?;
        Ok(data.get(key).cloned())
    }

    pub fn update(&self, key: String, resource: Resource<T>) -> Result<()> {
        let mut data = self.data.lock().map_err(|_| DatabaseError::LockError)?;
        data.insert(key, resource);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        let mut data = self.data.lock().map_err(|_| DatabaseError::LockError)?;
        data.remove(key);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn append_to_list(&self, key: &str, new_items: Vec<T>) -> Result<()> {
        let mut data = self.data.lock().map_err(|_| DatabaseError::LockError)?;

        match data.get_mut(key) {
            Some(Resource::List(list)) => {
                list.extend(new_items);
                self.save_to_disk()?;
                Ok(())
            }
            None => Err(DatabaseError::KeyNotFound(key.to_string())),
            _ => Err(DatabaseError::InvalidResourceType(key.to_string())),
        }
    }

    pub fn remove_from_list(&self, key: &str, item: T) -> Result<()> {
        let mut data = self.data.lock().map_err(|_| DatabaseError::LockError)?;

        match data.get_mut(key) {
            Some(Resource::List(list)) => {
                list.retain(|existing_item| *existing_item != item);
                self.save_to_disk()?;
                Ok(())
            }
            None => Err(DatabaseError::KeyNotFound(key.to_string())),
            _ => Err(DatabaseError::InvalidResourceType(key.to_string())),
        }
    }
}

impl<T> Server<T>
where
    T: Serialize + for<'a> Deserialize<'a> + PartialEq + Clone + Send + Sync + 'static,
{
    pub async fn new(addr: &str, db: Database<T>) -> Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        let sock_addr = socket.local_addr()?;
        let (shutdown_sender, _) = broadcast::channel(1);

        Ok(Server {
            socket: Arc::new(socket),
            db: Arc::new(db),
            shutdown: Arc::new(shutdown_sender),
            addr: sock_addr,
            handler: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let mut shutdown_rx = self.shutdown.subscribe();
        let (response_tx, mut response_rx) = mpsc::channel::<(String, SocketAddr)>(32);

        let socket = self.socket.clone();
        let response_handler = task::spawn(async move {
            while let Some((response, addr)) = response_rx.recv().await {
                if let Err(e) = socket.send_to(response.as_bytes(), addr).await {
                    error!("Failed to send response: {}", e);
                }
            }
        });

        let socket = self.socket.clone();
        let db = self.db.clone();
        let receive_handler = task::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                tokio::select! {
                    result = socket.recv_from(&mut buf) => {
                        match result {
                            Ok((len, addr)) => {
                                let request = String::from_utf8_lossy(&buf[..len]).to_string();
                                let response = handle_request(&db, &request);
                                if let Err(_) = response_tx.send((response, addr)).await {
                                    break;
                                }
                            }
                            Err(e) => {
                                error!("Failed to receive data: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
        });

        // Store the handler for cleanup
        if let Ok(mut handler) = self.handler.lock() {
            *handler = Some(receive_handler);
        }

        tokio::select! {
            _ = shutdown_rx.recv() => {
                info!("Shutting down server...");
            }
            _ = response_handler => {
                error!("Response handler terminated unexpectedly");
            }
        }

        Ok(())
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown.send(());

        // Cleanup the handler
        if let Ok(mut handler) = self.handler.lock() {
            if let Some(h) = handler.take() {
                h.abort();
            }
        }
    }

    pub async fn run_with_timeout(&self, duration: Duration) -> Result<()> {
        let run_future = self.run();
        match timeout(duration, run_future).await {
            Ok(result) => result,
            Err(_) => {
                self.shutdown();
                Ok(())
            }
        }
    }
}

// Add helper function for tests to get random port
pub fn get_random_port() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(49152..65535) // Dynamic port range
}

// Add helper function to check if port is available
pub async fn is_port_available(port: u16) -> bool {
    UdpSocket::bind(format!("127.0.0.1:{}", port)).await.is_ok()
}

// Add helper function to get next available port
pub async fn get_next_available_port() -> u16 {
    let mut port = get_random_port();
    while !is_port_available(port).await {
        port = get_random_port();
    }
    port
}

pub fn handle_request<T>(db: &Arc<Database<T>>, request: &str) -> String
where
    T: Serialize + for<'a> Deserialize<'a> + PartialEq + Clone,
{
    let parts: Vec<&str> = request.splitn(3, ' ').collect();
    if parts.len() < 2 {
        return "Invalid command".to_string();
    }

    let command = parts[0].to_uppercase();
    let key = parts[1];
    let payload = parts.get(2).cloned().unwrap_or("");

    match command.as_str() {
        "CREATE" => match serde_json::from_str::<Resource<T>>(payload) {
            Ok(resource) => match db.create(key.to_string(), resource) {
                Ok(_) => "Created successfully".to_string(),
                Err(e) => format!("Error: {}", e),
            },
            Err(_) => "Invalid data format".to_string(),
        },
        "READ" => match db.read(key) {
            Ok(Some(resource)) => serde_json::to_string(&resource).unwrap_or("Error".to_string()),
            Ok(None) => "Key not found".to_string(),
            Err(_) => "Error".to_string(),
        },
        "UPDATE" => match serde_json::from_str::<Resource<T>>(payload) {
            Ok(resource) => match db.update(key.to_string(), resource) {
                Ok(_) => "Updated successfully".to_string(),
                Err(e) => format!("Error: {}", e),
            },
            Err(_) => "Invalid data format".to_string(),
        },
        "DELETE" => match db.delete(key) {
            Ok(_) => "Deleted successfully".to_string(),
            Err(_) => "Error".to_string(),
        },
        "APPEND" => match serde_json::from_str::<Vec<T>>(payload) {
            Ok(new_items) => match db.append_to_list(key, new_items) {
                Ok(_) => "Appended successfully".to_string(),
                Err(e) => format!("Error: {}", e),
            },
            Err(_) => "Invalid data format".to_string(),
        },
        "REMOVE" => match serde_json::from_str::<T>(payload) {
            Ok(item) => match db.remove_from_list(key, item) {
                Ok(_) => "Removed successfully".to_string(),
                Err(e) => format!("Error: {}", e),
            },
            Err(_) => "Invalid data format".to_string(),
        },
        _ => "Unknown command".to_string(),
    }
}

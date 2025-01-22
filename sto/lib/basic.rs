use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::net::UdpSocket;
use tokio::sync::broadcast;
use tokio::task;

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
pub enum Resource {
    Single(Human),
    List(Vec<Human>),
}

pub type Db = HashMap<String, Resource>;

pub const DB_FILE: &str = "db.json";

#[derive(Debug)]
pub struct Database {
    data: Mutex<Db>,
    file_path: String,
}

impl Database {
    pub fn new(file_path: Option<&str>) -> Result<Self> {
        let path = match file_path {
            Some(":memory:") => String::new(), // Empty string for in-memory
            Some(path) => path.to_string(),
            None => DB_FILE.to_string(),
        };

        let data = if !path.is_empty() && std::path::Path::new(&path).exists() {
            let mut file = File::open(&path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            serde_json::from_str(&content).unwrap_or_else(|e| {
                warn!("Failed to parse database file: {}", e);
                HashMap::new()
            })
        } else {
            HashMap::new()
        };

        Ok(Database {
            data: Mutex::new(data),
            file_path: path,
        })
    }

    fn save_to_disk(&self) -> Result<()> {
        // Don't save if using in-memory database
        if self.file_path.is_empty() {
            return Ok(());
        }

        let data = self.data.lock().map_err(|_| DatabaseError::LockError)?;
        let json = serde_json::to_string(&*data)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn create(&self, key: String, resource: Resource) -> Result<()> {
        let mut data = self.data.lock().map_err(|_| DatabaseError::LockError)?;
        data.insert(key, resource);
        self.save_to_disk()?;
        Ok(())
    }

    pub fn read(&self, key: &str) -> Result<Option<Resource>> {
        let data = self.data.lock().map_err(|_| DatabaseError::LockError)?;
        Ok(data.get(key).cloned())
    }

    pub fn update(&self, key: String, resource: Resource) -> Result<()> {
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

    pub fn append_to_list(&self, key: &str, new_items: Vec<Human>) -> Result<()> {
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

    pub fn remove_from_list(&self, key: &str, item_name: &str) -> Result<()> {
        let mut data = self.data.lock().map_err(|_| DatabaseError::LockError)?;

        match data.get_mut(key) {
            Some(Resource::List(list)) => {
                list.retain(|human| human.name != item_name);
                self.save_to_disk()?;
                Ok(())
            }
            None => Err(DatabaseError::KeyNotFound(key.to_string())),
            _ => Err(DatabaseError::InvalidResourceType(key.to_string())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Server {
    socket: Arc<UdpSocket>,
    db: Arc<Database>,
    shutdown: Arc<broadcast::Sender<()>>,
}

impl Server {
    pub async fn new(addr: &str, db: Database) -> Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        let (shutdown_sender, _) = broadcast::channel(1);

        Ok(Server {
            socket: Arc::new(socket),
            db: Arc::new(db),
            shutdown: Arc::new(shutdown_sender),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let mut shutdown_rx = self.shutdown.subscribe();

        loop {
            let mut buf = vec![0; 1024];
            tokio::select! {
                result = self.socket.recv_from(&mut buf) => {
                    match result {
                        Ok((len, addr)) => {
                            let db = self.db.clone();
                            let socket = self.socket.clone();
                            let request = String::from_utf8_lossy(&buf[..len]).to_string();
                            info!("Received request: {}", request);

                            task::spawn(async move {
                                let response = handle_request(&db, &request);
                                info!("Sending response: {}", response);
                                if let Err(e) = socket.send_to(response.as_bytes(), &addr).await {
                                    error!("Failed to send response: {}", e);
                                }
                            });
                        }
                        Err(e) => error!("Failed to receive data: {}", e),
                    }
                }
                _ = shutdown_rx.recv() => {
                    info!("Shutting down server...");
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown.send(());
    }

    // Add timeout helper for tests
    // #[cfg(test)]
    pub async fn run_with_timeout(&self, duration: std::time::Duration) -> Result<()> {
        tokio::select! {
            result = self.run() => result,
            _ = tokio::time::sleep(duration) => Ok(()),
        }
    }
}

pub fn handle_request(db: &Arc<Database>, request: &str) -> String {
    let parts: Vec<&str> = request.splitn(3, ' ').collect();
    if parts.len() < 2 {
        return "Invalid command".to_string();
    }

    let command = parts[0].to_uppercase();
    let key = parts[1];
    let payload = parts.get(2).cloned().unwrap_or("");

    match command.as_str() {
        "CREATE" => match serde_json::from_str::<Resource>(payload) {
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
        "UPDATE" => match serde_json::from_str::<Resource>(payload) {
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
        _ => "Unknown command".to_string(),
    }
}

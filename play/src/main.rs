use base64::{self, Engine};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Meta {
    id: String,
    updated_at: u64,
}

#[derive(Debug, Clone)]
pub struct Human {
    name: String,
    age: u8,
}

#[derive(Debug, Clone)]
pub struct LogMessage {
    id: String,
    thread_id: usize,
    message: String,
    is_stdout: bool,
    // timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct StateObjectHuman {
    meta: Meta,
    human: Human,
    log_sender: Sender<LogMessage>,
    thread_id: usize,
}

impl StateObjectHuman {
    pub fn new(name: &str, age: u8, thread_id: usize, log_sender: Sender<LogMessage>) -> Self {
        let id = base64::engine::general_purpose::STANDARD.encode(Uuid::new_v4().as_bytes());
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        StateObjectHuman {
            meta: Meta {
                id: id.clone(),
                updated_at: now,
            },
            human: Human {
                name: name.to_string(),
                age,
            },
            log_sender,
            thread_id,
        }
    }

    /// Send a log message to the logger thread
    pub fn log(&self, message: &str, is_stdout: bool) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let log_message = LogMessage {
            id: self.meta.id.clone(),
            thread_id: self.thread_id,
            message: message.to_string(),
            is_stdout,
        //    timestamp,
        };
        self.log_sender.send(log_message).unwrap();
    }

    /// Update human object and log the change
    pub fn update(&mut self, name: Option<&str>, age: Option<u8>) {
        if let Some(name) = name {
            self.human.name = name.to_string();
            self.log(&format!("Updated name to: {}", self.human.name), true);
        }
        if let Some(age) = age {
            self.human.age = age;
            self.log(&format!("Updated age to: {}", self.human.age), true);
        }
        self.meta.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.log(
            &format!("Updated meta: updated_at = {}", self.meta.updated_at),
            true,
        );
    }

    /// Print log contents from stdout and stderr files
    pub fn print_logs(&self) {
        let stdout_path = format!(
            "target/logs/thread_{}_stdout_{}.log",
            self.thread_id, self.meta.updated_at
        );
        let stderr_path = format!(
            "target/logs/thread_{}_stderr_{}.log",
            self.thread_id, self.meta.updated_at
        );

        println!("Logs for {}:", self.meta.id);

        // Read and print stdout logs
        if let Ok(mut file) = File::open(&stdout_path) {
            println!("--- stdout ---");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            println!("{}", contents);
        }

        // Read and print stderr logs
        if let Ok(mut file) = File::open(&stderr_path) {
            println!("--- stderr ---");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            eprintln!("{}", contents);
        }
    }

    /// Delete stdout and stderr logs
    pub fn delete_logs(&self) {
        let stdout_path = format!(
            "target/logs/thread_{}_stdout_{}.log",
            self.thread_id, self.meta.updated_at
        );
        let stderr_path = format!(
            "target/logs/thread_{}_stderr_{}.log",
            self.thread_id, self.meta.updated_at
        );

        if Path::new(&stdout_path).exists() {
            std::fs::remove_file(stdout_path).unwrap();
            self.log("Deleted stdout log", true);
        }
        if Path::new(&stderr_path).exists() {
            std::fs::remove_file(stderr_path).unwrap();
            self.log("Deleted stderr log", false);
        }
    }
}

/// Logger thread function that writes log messages to stdout or stderr files
fn logger_thread(receiver: Receiver<LogMessage>) {
    while let Ok(log_message) = receiver.recv() {
        let log_file_path = if log_message.is_stdout {
            // format!(
            //     "target/logs/thread_{}_stdout_{}.log",
            //     log_message.thread_id, log_message.timestamp
            // )
            format!(
                "target/logs/thread_{}_stdout.log",
                log_message.thread_id//, log_message.timestamp
            )
        } else {
            // format!(
            //     "target/logs/thread_{}_stderr_{}.log",
            //     log_message.thread_id, log_message.timestamp
            // )
            format!(
                "target/logs/thread_{}_stderr.log",
                log_message.thread_id//, log_message.timestamp
            )
        };

        let log_path = Path::new(&log_file_path);
        let mut log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_path)
            .unwrap();

        writeln!(log_file, "{}", log_message.message).unwrap();
    }
}

fn main() {
    // Ensure the target/logs directory exists
    std::fs::create_dir_all("target/logs").unwrap();

    // Create crossbeam channel for logging
    let (log_sender, log_receiver): (Sender<LogMessage>, Receiver<LogMessage>) = unbounded();

    // Spawn the logger thread
    let logger_handle = thread::spawn(move || {
        logger_thread(log_receiver);
    });

    // Setup example with logging in separate threads
    let humans: Vec<(&str, u8)> = vec![
        ("mariah", 18),
        ("hannah", 18),
        ("sarah", 18),
        ("elizabeth", 18),
        ("mary", 18),
        ("lucy", 18),
        ("cynthia", 18),
        ("josephina", 18),
        ("sofia", 18),
        ("jessica", 18),
        ("marasen", 18),
        ("clara", 18),
    ];

    let mut handles = vec![];

    for (i, (name, age)) in humans.into_iter().enumerate() {
        let log_sender = log_sender.clone();

        let handle = thread::spawn(move || {
            let mut human = StateObjectHuman::new(name, age, i, log_sender.clone());

            human.update(Some(&format!("{} Updated", name)), Some(age + 1));
            human.print_logs();
            human.delete_logs();
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Join logger thread
    logger_handle.join().unwrap();
}


// cargo watch -x run
// suspend cargo watch zsh job by `ctrl+z`
// display suspended zsh jobs by `fg`
// select job using `%1 or %N`
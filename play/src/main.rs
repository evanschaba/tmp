use lazy_static::lazy_static;
use std::{
    fs::OpenOptions,
    io::{self, Write},
    sync::Mutex,
};

lazy_static! {
    static ref LOGGER: Mutex<io::BufWriter<std::fs::File>> = Mutex::new(io::BufWriter::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open("target/logs/traces.log")
            .expect("Failed to create log file")
    ));
}

#[derive(Debug, Clone)]
enum Command {
    Run(i32, i32),
    PrintResultSum(i32, i32),
    Sum(i32, i32),
    Add(i32, i32),
}

struct StateMachine {
    depth: usize,
}

impl StateMachine {
    fn new() -> Self {
        Self { depth: 0 }
    }

    fn log(&self, msg: &str) -> io::Result<()> {
        let mut logger = LOGGER.lock().unwrap();
        let indentation = " ".repeat(self.depth * 4);
        writeln!(logger, "{}{}", indentation, msg)?;
        logger.flush()
    }

    fn execute(&mut self, command: Command) -> io::Result<()> {
        self.depth = 0; // Reset depth for each new command from main
        self.run_command(command)
    }

    fn run_command(&mut self, command: Command) -> io::Result<()> {
        match command {
            Command::Run(a, b) => {
                self.log(&format!("run({}, {})", a, b))?;
                self.depth += 1;
                self.print_result_sum(a, b)?;
                self.depth -= 1;
            }
            Command::PrintResultSum(a, b) => {
                self.log(&format!("print_sum({}, {})", a, b))?;
                self.depth += 1;
                self.sum(a, b)?;
                self.depth -= 1;
            }
            Command::Sum(a, b) => {
                self.log(&format!("sum({}, {})", a, b))?;
                self.depth += 1;
                self.add(a, b)?;
                self.depth -= 1;
            }
            Command::Add(a, b) => {
                let result = a + b;
                self.log(&format!("add({}, {}) -> {}", a, b, result))?;
            }
        }
        Ok(())
    }

    fn print_result_sum(&self, a: i32, b: i32) -> io::Result<()> {
        self.log(&format!("print_result_sum({}, {})", a, b))?;
        self.sum(a, b)?;
        Ok(())
    }

    fn sum(&self, a: i32, b: i32) -> io::Result<()> {
        let result = a + b;
        self.log(&format!("sum({}, {}) -> {}", a, b, result))?;
        Ok(())
    }

    fn add(&self, a: i32, b: i32) -> io::Result<()> {
        let result = a + b;
        self.log(&format!("add({}, {}) -> {}", a, b, result))?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut sm = StateMachine::new();
    sm.execute(Command::Run(1, 1))?;
    Ok(())
}

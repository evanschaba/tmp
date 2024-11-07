use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::Mutex;
use chrono::Local;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref LOGGER: Mutex<File> = Mutex::new(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("target/logs/{}.log", Local::now().format("%Y%m%d-%H%M%S")))
            .expect("Failed to create log file")
    );
}

#[derive(Debug, Clone)]
enum Command {
    Run(i32, i32),
    PrintResultSum(i32, i32),
    Sum(i32, i32),
}

struct StateMachine {
    state: i32,
    history: Vec<Command>,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            state: 0,
            history: Vec::new(),
        }
    }

    fn log(&self, msg: &str) {
        let mut logger = LOGGER.lock().unwrap();
        writeln!(logger, "{}", msg).unwrap();
    }

    fn execute(&mut self, command: Command) -> i32 {
        self.history.push(command.clone());
        self.log(&format!("Executing: {:?}", command));
        
        match command {
            Command::Run(a, b) => self.run(a, b),
            Command::PrintResultSum(a, b) => self.print_result_sum(a, b),
            Command::Sum(a, b) => self.sum(a, b),
        }
    }

    fn run(&mut self, a: i32, b: i32) -> i32 {
        self.log("main()");
        let result = self.print_result_sum(a, b);
        self.log(&format!("main -> run({}, {}) -> result: {}\n", a, b, result));
        result
    }

    fn print_result_sum(&mut self, a: i32, b: i32) -> i32 {
        self.log(&format!("    run -> print_result_sum({}, {})", a, b));
        let result = self.sum(a, b);
        self.log(&format!("        print_result_sum -> sum({}, {}) -> result: {}\n", a, b, result));
        result
    }

    fn sum(&mut self, a: i32, b: i32) -> i32 {
        let result = a + b;
        self.log(&format!("            sum({}, {}) -> {}", a, b, result));
        result
    }

    fn replay(&mut self) {
        for command in &self.history {
            self.execute(command.clone());
        }
    }
}

fn main() -> io::Result<()> {
    let mut sm = StateMachine::new();
    sm.execute(Command::Run(0, 0));

    sm.log("Starting replay...");
    sm.replay();

    Ok(())
}

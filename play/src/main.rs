use tracer::trace_and_log;

#[trace_and_log]
fn a() {
    println!("Executing function a");
}

#[trace_and_log]
pub fn sum(x: u8, y: u8) -> u8 {
    x + y
}

#[trace_and_log]
fn main() {
    std::fs::create_dir_all("target/logs").unwrap();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    a();
    let result = sum(5, 3);
    println!("Sum result: {}", result);
}

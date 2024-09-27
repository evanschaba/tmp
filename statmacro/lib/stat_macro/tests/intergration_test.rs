use std::io::{self, Write};

use stat_macro::*;

#[derive(Stats)] // applicable to structs
pub struct Example {
    pub a: u32,
    pub b: f64,
    #[stats]
    // also applicable to struct-fields: Apply the custom attribute to generate memory stats for this larger field
    pub c: [u8; 128],
}

fn capture_output<F>(func: F) -> String
where
    F: FnOnce(),
{
    let mut buf = Vec::new();
    let writer = io::stdout();
    let mut _guard = writer.lock(); // Make this mutable
    func();
    _guard.flush().unwrap();
    buf.clear(); // Reset buffer

    String::from_utf8(buf).unwrap_or_default()
}

#[test]
fn test_memory_address() {
    let instance = Example {
        a: 42,
        b: std::f64::consts::PI,
        c: [0; 128],
    };
    let output = capture_output(|| instance.print_memory_address());
    assert!(output.contains("Memory address of Example: "));
}

#[test]
fn test_size() {
    let instance = Example {
        a: 42,
        b: std::f64::consts::PI,
        c: [0; 128],
    };
    let output = capture_output(|| instance.print_size());
    assert!(output.contains("Size of Example: "));
}

#[test]
fn test_field_stats() {
    let instance = Example {
        a: 42,
        b: std::f64::consts::PI,
        c: [0; 128],
    };
    let output = capture_output(|| instance.print_field_stats());
    assert!(output.contains("Field `c`: memory address: "));
}

#[test]
fn test_non_stats_field() {
    let instance = Example {
        a: 42,
        b: std::f64::consts::PI,
        c: [0; 128],
    };
    let output = capture_output(|| instance.print_field_stats());
    assert!(!output.contains("Field `a`: memory address:  "));
}

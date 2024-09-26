use stat_macro::Stats;

#[derive(Stats)] // applicable to structs
pub struct Example {
    pub a: u32,
    pub b: f64,
    #[stats]
    // also applicable to struct-fields: Apply the custom attribute to generate memory stats for this larger field
    pub c: [u8; 128],
}

// why do i need this? well, dbg tracing brah

fn main() {
    let instance = Example {
        a: 42,
        b: 3.14,
        c: [0; 128],
    };

    // Print memory stats of the struct
    instance.print_memory_address();
    instance.print_size();

    // Print memory stats for fields with #[stats]
    instance.print_field_stats();
}

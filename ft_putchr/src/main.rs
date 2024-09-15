use tmp::ft_putchr::ft_putchr;
use tmp::ft_putchrs::ft_putchrs;

fn main() {
    match ft_putchr('A') {
        Ok(bytes) => println!("\n{} bytes\n", bytes),
        Err(e) => eprintln!("Error: {}", e),
    }

    match ft_putchrs("hello world") {
        Ok(bytes) => println!("\n{} bytes\n", bytes),
        Err(e) => eprintln!("Error: {}", e),
    }

    let chars_as_bytes = &[108, 111, 118, 101];

    match ft_putchrs(chars_as_bytes) {
        Ok(bytes) => println!("\n{} bytes\n", bytes),
        Err(e) => eprintln!("Error: {}", e),
    }
}

use libft::ft_putchr::ft_putchr;
use libft::ft_putchrs::ft_putchrs;

fn main() {
    // Test with a character
    match ft_putchr('A') {
        Ok(bytes) => println!("\n{} bytes\n", bytes),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Test with a string
    match ft_putchrs("hello world") {
        Ok(bytes) => println!("\n{} bytes\n", bytes),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Test with ASCII bytes for "love"
    let chars_as_bytes = &[108, 111, 118, 101];

    match ft_putchrs(chars_as_bytes) {
        Ok(bytes) => println!("\n{} bytes\n", bytes),
        Err(e) => eprintln!("Error: {}", e),
    }
}

use std::io::{self, Write};

// Internal function that handles actual writing and conversion
fn _ft_putchr_impl(c: FtChar) -> io::Result<usize> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let char_to_print = match c {
        FtChar::Char(ch) => ch,
        FtChar::Int(n) => n as char, // Convert the integer to its ASCII character
    };

    // Write the character as UTF-8 bytes
    let mut buffer = [0; 4]; // Buffer to store UTF-8 representation of the char
    let encoded = char_to_print.encode_utf8(&mut buffer);

    handle.write_all(encoded.as_bytes())?;
    handle.flush()?;

    Ok(encoded.len()) // Return the number of bytes written
}

// Enum to handle both characters and integers
#[derive(Debug, Clone, Copy)] // Added traits for debugging and cloning
pub enum FtChar {
    Char(char),
    Int(u8), // ASCII range for integers (0-255)
}

// Implement conversions (From trait) for simplicity
impl From<char> for FtChar {
    fn from(c: char) -> Self {
        FtChar::Char(c)
    }
}

impl From<u8> for FtChar {
    fn from(n: u8) -> Self {
        FtChar::Int(n)
    }
}

// API wrapper
pub fn ft_putchr<T: Into<FtChar>>(input: T) -> io::Result<usize> {
    let ft_char = input.into(); // Convert the input into FtChar enum
    _ft_putchr_impl(ft_char) // Call the internal function
}

fn main() {
    // Test with a character
    match ft_putchr('A') {
        Ok(bytes) => println!("\nbytes_written: {}\n", bytes),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Test with an ASCII value
    match ft_putchr(69) {
        Ok(bytes) => println!("\nbytes_written: {}\n", bytes),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ft_putchr_char() {
        let result = ft_putchr('B').expect("Failed to write character\n");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_ft_putchr_int() {
        let result = ft_putchr(66_u8).expect("Failed to write integer\n");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_ft_putchr_invalid_char() {
        // This test is to ensure proper handling of non-ASCII chars
        // For this example, it's just checking if no error occurs.
        let result = ft_putchr('ç').expect("Failed to write character\n");
        assert_eq!(result, 2); // Assuming 'ç' is represented with 2 bytes
    }
}

use std::io::{self, Write};

// Private function that handles actual writing
fn _ft_putchar_impl(c: char) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Write the character as UTF-8 bytes
    let mut buffer = [0; 4]; // Buffer to store UTF-8 representation of the char
    let encoded = c.encode_utf8(&mut buffer);

    handle.write_all(encoded.as_bytes())?;
    handle.flush()?;

    Ok(())
}

// Public wrapper function to keep the API simple
pub fn ft_putchar<T: Into<FtChar>>(input: T) -> io::Result<()> {
    let ft_char = input.into(); // Convert the input into FtChar enum
    let char_to_print = match ft_char {
        FtChar::Char(ch) => ch,
        FtChar::Int(n) => n as char, // Convert the integer to its ASCII character
    };

    _ft_putchar_impl(char_to_print)
}

// Enum to handle both characters and integers
enum FtChar {
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

fn main() {
    // API stays simple, user doesn't need to care about implementation details
    ft_putchar('A').unwrap();    // Using a char
    ft_putchar(65_u8).unwrap();  // Using an ASCII value for 'A'
}

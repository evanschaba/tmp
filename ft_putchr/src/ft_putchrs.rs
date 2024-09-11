use crate::ft_putchr::ft_putchr;
use std::io;

/// Enum to wrap either a string or a byte slice
pub enum FtChrsInput<'a> {
    Str(&'a str),
    Bytes(&'a [u8]),
}

/// Implement `Into` for `FtChrsInput` to convert `&str` and `&[u8]` into `FtChrsInput`
impl<'a> From<&'a str> for FtChrsInput<'a> {
    fn from(val: &'a str) -> Self {
        FtChrsInput::Str(val)
    }
}

impl<'a> From<&'a [u8]> for FtChrsInput<'a> {
    fn from(val: &'a [u8]) -> Self {
        FtChrsInput::Bytes(val)
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for FtChrsInput<'a> {
    fn from(val: &'a [u8; N]) -> Self {
        FtChrsInput::Bytes(val)
    }
}

/// Function to handle both string and byte slice inputs and return the total number of bytes written
pub fn ft_putchrs<T: Into<FtChrsInput<'static>>>(input: T) -> io::Result<usize> {
    let mut bytes_written: usize = 0;
    let input = input.into();

    match input {
        FtChrsInput::Str(chrs) => {
            for c in chrs.chars() {
                match ft_putchr(c) {
                    Ok(bytes) => bytes_written += bytes, // Accumulate bytes written
                    Err(e) => eprintln!("Error: {}", e), // Print error but continue
                }
            }
        }
        FtChrsInput::Bytes(bytes) => {
            for &byte in bytes {
                let char_to_write = byte as char;
                match ft_putchr(char_to_write) {
                    Ok(written) => bytes_written += written, // Accumulate bytes written
                    Err(e) => eprintln!("Error: {}", e),     // Print error but continue
                }
            }
        }
    }

    Ok(bytes_written) // Return the total number of bytes written
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};

    #[test]
    fn test_ft_putchrs_from_str() {
        let result = ft_putchrs("hello world").expect("Failed to write string");
        assert_eq!(result, 11); // "hello world" has 11 characters, so total bytes written should be 11
    }

    #[test]
    fn test_ft_putchrs_from_bytes() {
        let bytes: &[u8] = &[72, 101, 108, 108, 111]; // Corresponds to 'H', 'e', 'l', 'l', 'o'
        let result = ft_putchrs(bytes).expect("Failed to write bytes");
        assert_eq!(result, 5); // 'Hello' has 5 characters, so total bytes written should be 5
    }

    #[test]
    fn test_ft_putchrs_from_array() {
        let bytes: &[u8; 5] = &[72, 101, 108, 108, 111]; // Corresponds to 'H', 'e', 'l', 'l', 'o'
        let result = ft_putchrs(bytes).expect("Failed to write bytes");
        assert_eq!(result, 5); // 'Hello' has 5 characters, so total bytes written should be 5
    }

    // A modified version of the ft_putchrs function that writes to any writer
    fn ft_putchrs_with_writer<T: Into<FtChrsInput<'static>>, W: Write>(
        input: T,
        writer: &mut W,
    ) -> io::Result<usize> {
        let mut bytes_written: usize = 0;
        let input = input.into();

        match input {
            FtChrsInput::Str(chrs) => {
                for c in chrs.chars() {
                    writer.write_all(&[c as u8])?;
                    bytes_written += 1;
                }
            }
            FtChrsInput::Bytes(bytes) => {
                writer.write_all(bytes)?;
                bytes_written += bytes.len();
            }
        }

        writer.flush()?;
        Ok(bytes_written)
    }

    fn _ft_putchrs_with_cursor<T: Into<FtChrsInput<'static>> + Clone>(input: T) {
        // Create a Cursor buf to capture the output
        let mut buf = Cursor::new(Vec::new());

        // Redirect output to the buf instead of stdout
        let _written =
            ft_putchrs_with_writer(input.clone(), &mut buf).expect("Failed to write input");

        // Convert the buf content to a string (in case of valid UTF-8 data)
        let output = String::from_utf8(buf.into_inner()).expect("Failed to convert buf to string");

        // Assert that the output matches the expected string
        match input.into() {
            FtChrsInput::Str(expected_str) => {
                assert_eq!(output, expected_str);
            }
            FtChrsInput::Bytes(expected_bytes) => {
                assert_eq!(output.as_bytes(), expected_bytes);
            }
        }
    }

    #[test]
    fn test_ft_putchrs_with_cursor() {
        _ft_putchrs_with_cursor("hello world");
        _ft_putchrs_with_cursor(&[72, 101]);

        // "hello"
        _ft_putchrs_with_cursor(&[
            ('A' as u8 + 32 + 7),  // 'h': 65 + 7 = 72 (ASCII 'H') + 32 = 'h'
            ('A' as u8 + 32 + 4),  // 'e': 65 + 4 = 69 (ASCII 'E') + 32 = 'e'
            ('A' as u8 + 32 + 11), // 'l': 65 + 11 = 76 (ASCII 'L') + 32 = 'l'
            ('A' as u8 + 32 + 11), // 'l': 65 + 11 = 76 (ASCII 'L') + 32 = 'l'
            ('A' as u8 + 32 + 14), // 'o': 65 + 14 = 79 (ASCII 'O') + 32 = 'o'
        ]);

        // "HELLO"
        _ft_putchrs_with_cursor(&[
            ((('A' as u8) + 7) % 128),  // 'h': 65 + 7 = 72 (ASCII 'H') + 32 = 'h'
            ((('A' as u8) + 4) % 128),  // 'e': 65 + 4 = 69 (ASCII 'E') + 32 = 'e'
            ((('A' as u8) + 11) % 128), // 'l': 65 + 11 = 76 (ASCII 'L') + 32 = 'l'
            ((('A' as u8) + 11) % 128), // 'l': 65 + 11 = 76 (ASCII 'L') + 32 = 'l'
            ((('A' as u8) + 14) % 128), // 'o': 65 + 14 = 79 (ASCII 'O') + 32 = 'o'
        ]);
    }
}

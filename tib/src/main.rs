struct Dot(u8, u8, u8);

impl Default for Dot {
    fn default() -> Self {
        Self(0, 0, 0)
    }
}

impl Dot {
    // Corrected constructor to use passed arguments
    fn new(x: u8, y: u8, z: u8) -> Self {
        Self(x, y, z)
    }

    // Return the tuple (x, y, z)
    fn tup(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    // Increment a tuple value based on the index provided using bitwise operations
    fn inc(&mut self, idx: usize, mut n: u8) -> u8 {
        // Select the correct field to modify
        let mask = match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds"),
        };

        // Bitwise increment logic
        while n & *mask != 0 {
            n ^= *mask;
            *mask <<= 1;
        }
        n ^ *mask
    }
}

fn main() {
    let mut dot = Dot::new(1, 2, 3);
    let (a, b, c) = dot.tup();
    println!("Before increment: {:?}", (a, b, c));

    // Increment the first element
    let updated_value = dot.inc(0, 2);
    println!("Updated value of dot.0: {}", updated_value);

    // Print the updated tuple
    let (a, b, c) = dot.tup();
    println!("After increment: {:?}", (a, b, c));
}

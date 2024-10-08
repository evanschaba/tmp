use std::sync::{Arc, Mutex};
use std::thread;
use std::error::Error;
use std::fmt::Debug;

/// A macro to create a thread-safe version of any struct by wrapping each field in `Arc<Mutex<>>`.
macro_rules! thread_safe_struct {
    // Define a thread-safe version of the struct with fields wrapped.
    ($struct_name:ident { $($field:ident : $type:ty),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $struct_name {
            $(pub $field: Arc<Mutex<$type>>),*
        }

        impl $struct_name {
            // Constructor to create the struct and initialize each field.
            pub fn new($($field: $type),*) -> Self {
                Self {
                    $($field: Arc::new(Mutex::new($field))),*
                }
            }

            // Thread-safe getter for a field.
            $(pub fn get_$field(&self) -> Result<$type, Box<dyn Error>> where $type: Copy {
                let value = self.$field.lock()?;
                Ok(*value)
            })*

            // Thread-safe setter for a field.
            $(pub fn set_$field(&self, new_value: $type) -> Result<(), Box<dyn Error>> {
                let mut value = self.$field.lock()?;
                *value = new_value;
                Ok(())
            })*

            // Multi-threaded initialization for each field.
            pub fn multi_threaded_init(&self) -> Result<(), Box<dyn Error>> {
                let mut handles = vec![];

                // Spawn threads to initialize/update each field concurrently.
                $(
                    let field_clone = Arc::clone(&self.$field);
                    let handle = thread::spawn(move || {
                        let mut value = field_clone.lock().unwrap();
                        *value += 1; // Example thread modification
                        println!("Updated field {} to {}", stringify!($field), *value);
                    });
                    handles.push(handle);
                )*

                for handle in handles {
                    handle.join().unwrap();
                }

                Ok(())
            }
        }
    };
}

// Example usage of the macro to define a thread-safe struct `MyStruct`.
thread_safe_struct!(MyStruct {
    value1: i32,
    value2: i32,
});

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new instance of the thread-safe struct.
    let my_struct = MyStruct::new(10, 20);

    // Multi-threaded initialization of fields.
    my_struct.multi_threaded_init()?;

    // Thread-safe access to fields.
    println!("Value1: {}", my_struct.get_value1()?);
    println!("Value2: {}", my_struct.get_value2()?);

    // Thread-safe modification of fields.
    my_struct.set_value1(100)?;
    my_struct.set_value2(200)?;

    println!("Updated Value1: {}", my_struct.get_value1()?);
    println!("Updated Value2: {}", my_struct.get_value2()?);

    Ok(())
}

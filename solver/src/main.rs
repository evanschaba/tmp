use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared array inside Arc<Mutex<>> for thread-safe mutable access.
    let arr = Arc::new(Mutex::new([0; 10]));

    // Create multiple threads for parallel processing.
    let mut handles = vec![];

    for i in 0..10 {
        let arr_clone = Arc::clone(&arr);

        let handle = thread::spawn(move || {
            let mut arr_locked = arr_clone.lock().unwrap();
            // Safely mutate the array in the current thread.
            arr_locked[i] += 2;
            println!("Thread {}: Value at {} is now {}", i, i, arr_locked[i]);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    // Final array output
    let final_array = arr.lock().unwrap();
    println!("Final array: {:?}", *final_array);
}

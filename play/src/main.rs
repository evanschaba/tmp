// use std::sync::{Arc, Mutex};
// use std::thread;

// //** each array index is not updated at the same time. (not parallel), why? cause each thread is trying to get the locked array first, thereby forming some ordered update for the locked arr values */
// fn main() {
//     // Shared array inside Arc<Mutex<>> for thread-safe mutable access.
//     let arr = Arc::new(Mutex::new([0; 10]));

//     // Get the length of the array
//     let arr_len = arr.lock().unwrap().len();

//     // Create multiple threads for parallel processing.
//     let mut handles = vec![];

//     for i in 0..arr_len {
//         let arr_clone = Arc::clone(&arr);

//         let handle = thread::spawn(move || {
//             let mut arr_locked = arr_clone.lock().unwrap();
//             // Safely mutate the array in the current thread.
//             arr_locked[i] += 2;
//             println!("Thread {}: Index: {}, Value at {} => {}", i, i, i, arr_locked[i]);
//         });

//         handles.push(handle);
//     }

//     // Wait for all threads to finish.
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // Final array output
//     let final_array = arr.lock().unwrap();
//     println!("Final array: {:?}", *final_array);
// }

/// 2. NOW THIS IS WHAT I HAVE FIGURED OUT ABOUT TRUE PARALLELISM
// use std::sync::atomic::{AtomicI32, Ordering};
// use std::sync::{Arc /*,Mutex*/};
// use std::thread;

// fn main() {
//     // Shared array with atomic elements for lock-free thread-safe mutation
//     let arr: Arc<Vec<AtomicI32>> = Arc::new((0..10).map(|_| AtomicI32::new(0)).collect());

//     // Get the length of the array
//     let arr_len = arr.len();

//     // Create multiple threads for parallel processing.
//     let mut handles = vec![];

//     for i in 0..arr_len {
//         // Clone the reference to the array for each thread.
//         let arr_clone = Arc::clone(&arr);

//         let handle = thread::spawn(move || {
//             // Perform a lock-free atomic operation on the array element.
//             arr_clone[i].fetch_add(2, Ordering::SeqCst);
//             println!(
//                 "Thread {}: Index: {}, Value at {} => {}",
//                 i,
//                 i,
//                 i,
//                 arr_clone[i].load(Ordering::SeqCst)
//             );
//         });

//         handles.push(handle);
//     }

//     // Wait for all threads to finish.
//     for handle in handles {
//         handle.join().expect("Thread panicked.");
//     }

//     // Print final array output.
//     let final_array: Vec<i32> = arr.iter().map(|elem| elem.load(Ordering::SeqCst)).collect();
//     println!("Final array: {:?}", final_array);
// }

/// 3. NOW USING THE AMAZING RAYON FOR SUPER EFFICIENT THREAD POOLING
use rayon::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

fn main() {
    // Shared array with atomic elements for lock-free thread-safe mutation
    let arr: Arc<Vec<AtomicI32>> = Arc::new((0..10).map(|_| AtomicI32::new(0)).collect());

    // Use Rayon to parallelize the update operations
    (0..arr.len()).into_par_iter().for_each(|i| {
        arr[i].fetch_add(2, Ordering::SeqCst); // Lock-free atomic increment
        println!(
            "Thread {}: Index: {}, Value at {} => {}",
            i,
            i,
            i,
            // arr[i].load(Ordering::SeqCst)
            arr[i].load(Ordering::Relaxed)
        );
    });

    // Print final array output.
    let final_array: Vec<i32> = arr.iter().map(|elem| elem.load(Ordering::SeqCst)).collect();
    println!("Final array: {:?}", final_array);
}

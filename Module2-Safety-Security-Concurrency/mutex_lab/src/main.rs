// Main: Implement Mutex and Thread Logic

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     // Use a Mutex to protect the data vector
//     let data = Arc::new(Mutex::new(vec![1, 2, 3]));

//     // Create multiple threads to increment values
//     let mut handles = vec![];

//     for i in 0..3 {
//         let data_clone = Arc::clone(&data);
//         let handle = thread::spawn(move || {
//             let mut data = data_clone.lock().unwrap();
//             data[i] += 1;
//             println!("Thread {} updated data to {:?}", i, *data);
//         });
//         handles.push(handle);
//     }

//     // Wait for all threads to complete
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Final data: {:?}", *data.lock().unwrap());
// }

// Challenge Solutions:
// 1. Modify the code to use RwLock instead of Mutex

// use std::sync::{Arc, RwLock};
// use std::thread;

// fn main() {
//     let data = Arc::new(RwLock::new(vec![1, 2, 3]));

//     let mut handles = vec![];

//     for i in 0..3 {
//         let data_clone = Arc::clone(&data);
//         let handle = thread::spawn(move || {
//             let mut data = data_clone.write().unwrap();
//             data[i] += 1;
//             println!("Thread {} updated data to {:?}", i, *data);
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Final data: {:?}", *data.read().unwrap());
// }

// 2. Handle Potential Deadlocks

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     let data1 = Arc::new(Mutex::new(0));
//     let data2 = Arc::new(Mutex::new(0));

//     let data1_clone = Arc::clone(&data1);
//     let data2_clone = Arc::clone(&data2);

//     let handle1 = thread::spawn(move || {
//         let _lock1 = data1_clone.lock().unwrap();
//         let _lock2 = data2_clone.lock().unwrap();
//         println!("Thread 1 acquired both locks");
//     });

//     let handle2 = thread::spawn(move || {
//         let _lock2 = data2.lock().unwrap();
//         let _lock1 = data1.lock().unwrap();
//         println!("Thread 2 acquired both locks");
//     });

//     handle1.join().unwrap();
//     handle2.join().unwrap();
// }

// 3. Extend Code with Conditional Variables

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let handle = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();
        println!("Notified main thread");
    });

    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        ready = cvar.wait(ready).unwrap();
    }
    println!("Main thread proceeding");
    handle.join().unwrap();
}

use std::sync::{Arc, Mutex};
use std::thread;

pub fn execute() {
    let shared_data = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..10 {
        let data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += i;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let data = shared_data.lock().unwrap();
    println!("Final data value: {}", *data);
}


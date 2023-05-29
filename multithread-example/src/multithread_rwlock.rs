use std::sync::{Arc, RwLock};
use std::thread;

pub fn execute() {
    let shared_data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let data_read_lock = Arc::clone(&shared_data);
    let read_thread = thread::spawn(move || {
        let data = data_read_lock.read().unwrap();
        println!("Read thread: {:?}", data);
    });

    let data_write_lock = Arc::clone(&shared_data);
    let write_thread = thread::spawn(move || {
        let mut data = data_write_lock.write().unwrap();
        data.push(4);
        println!("Write thread: {:?}", data);
    });

    read_thread.join().unwrap();
    write_thread.join().unwrap();
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

pub fn execute() {
    let filenames = vec![ r"src\files\file1.csv",r"src\files\file2.csv",r"src\files\file3.csv"];

    let mut handles = vec![];

    for filename in filenames {
        let handle = thread::spawn(move || {
            let file = File::open(filename).expect("Failed to open file");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("Thread {:?}: {}", thread::current().id(), line.unwrap());
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

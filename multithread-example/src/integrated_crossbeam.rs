use crossbeam::thread;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Person {
    name: String,
    age: u8,
}

pub fn execute() {

    let filenames = vec![r"src\files\file1.json",r"src\files\file2.json",r"src\files\file3.json"];

    let shared_data = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| {
        for filename in &filenames {
            let shared_data = Arc::clone(&shared_data);
            s.spawn(move |_| {
                let file = File::open(filename).unwrap();
                let reader = BufReader::new(file);
                let people: Vec<Person> = serde_json::from_reader(reader).unwrap();

                let mut data = shared_data.lock().unwrap();
                data.extend_from_slice(&people);
            });
        }
    })
    .unwrap();

    let data = shared_data.lock().unwrap();
    println!("{:?}", data);
}

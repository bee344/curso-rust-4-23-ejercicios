use std::fs::File;
use std::io::{BufRead, BufReader};
use crossbeam::thread;

pub fn execute() {
    let files = vec![ r"src\files\file1.csv",r"src\files\file2.csv",r"src\files\file3.csv"];

    thread::scope(|s| {
        for filename in &files {
            s.spawn(|_| {
                if let Ok(f) = File::open(filename.clone()) {
                    let reader = BufReader::new(f);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            println!("{}: {}", filename.clone(), line);
                        }
                    }
                }
            });
        }
    }).unwrap();
}
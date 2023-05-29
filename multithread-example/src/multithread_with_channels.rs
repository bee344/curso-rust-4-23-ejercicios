use std::thread;
use std::sync::mpsc;

pub fn execute() {
    let (sender, receiver) = mpsc::channel();

    for i in 0..10 {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            sender_clone.send(format!("Mensaje {}", i)).unwrap();
        });
    }

    for _ in 0..10 {
        let mensaje = receiver.recv().unwrap();
        println!("{}", mensaje);
    }
}
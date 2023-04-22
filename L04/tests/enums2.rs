// I AM NOT DONE

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: u64, y: u64 },
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
        println!("{:?}", message);
    }
}

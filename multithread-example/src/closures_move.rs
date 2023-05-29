pub fn execute() {
    let name = "Rust".to_string();

    let my_closure = move || {
        println!("Hello, {}!", name);
    };

    my_closure();
}

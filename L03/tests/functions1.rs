// Don't mind this for now :)
// I AM NOT DONE
fn call_me() {
    println!("Function called!");
}
#[cfg(test)]
mod tests {
    use crate::call_me;
    #[test]
    fn call_function() {
        call_me();
    }
}

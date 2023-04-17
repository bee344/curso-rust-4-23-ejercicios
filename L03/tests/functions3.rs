// Don't mind this for now :)
// I AM NOT DONE
fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::call_me;
    #[test]
    fn call_function() {
        call_me(23);
    }
}

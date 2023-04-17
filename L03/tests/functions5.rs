// I AM NOT DONE

fn square(num: i32) -> i32 {
    let result = num * num;
    result
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn call_function() {
        let answer = square(3);
        println!("The square of 3 is {}", answer);
    }
}

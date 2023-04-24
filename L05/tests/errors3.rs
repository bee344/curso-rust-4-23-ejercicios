// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?

// I AM NOT DONE

use std::num::ParseIntError;

#[cfg(test)]
mod tests {

    use super::*;

    pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
        let processing_fee = 1;
        let cost_per_item = 5;
        let qty = item_quantity.parse::<i32>();

        if let Ok(_) = qty {
            Ok(qty? * cost_per_item + processing_fee)
        } else {
            qty
        }
    }

    #[test]
    fn test_error() {
        let mut tokens = 100;
        let pretend_user_input = "8";

        let cost = total_cost(pretend_user_input);

        if let Ok(c) = cost {
            if c > tokens {
                println!("You can't afford that many!");
            } else {
                tokens -= c;
                println!("You now have {} tokens.", tokens);
            }
        } else {
            println!("{}", cost.unwrap_err().to_string())
        }
    }
}

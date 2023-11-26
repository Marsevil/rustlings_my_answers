// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

use std::error::Error;
use std::num::ParseIntError;

type Token = i32;

const PROCESSING_FEE: Token = 1;
const COST_PER_ITEM: Token = 5;

fn main() -> Result<(), Box<dyn Error>> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let qty = item_quantity.parse::<i32>()?;
    let total_cost = qty * COST_PER_ITEM + PROCESSING_FEE;

    Ok(total_cost)
}

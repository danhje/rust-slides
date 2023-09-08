#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
    function_four();
}

fn function_one() {
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

fn function_two() {
    /// Try to find an even number in the given sequence of numbers.
    fn find_even(input: Vec<i32>) -> Option<i32> {
        for num in input {
            if num % 2 == 0 {
                return Some(num);
            }
        }
        None
        // This None is not a standalone object like None in Python,
        // it is the None variant of the Option enum
    }

    let maybe_number = find_even(vec![1, 3, 5]);

    // maybe_number + 1;  // error[E0369]: cannot add `{integer}` to `Option<i32>`
}

fn function_three() {
    /// Try to divide two numbers. Return Err if the divisor is zero.
    fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
        if divisor == 0.0 {
            Err("Cannot divide by zero!".to_string())
        } else {
            Ok(dividend / divisor)
        }
    }

    let result = divide(2.0, 3.0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("{e}"),
    }
}

fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(dividend / divisor)
    }
}

fn function_four() -> Result<(), String> {
    // Get the value out of the Ok variant,
    // or panic if the result is Err
    let result = divide(2.0, 3.0).unwrap();

    // Get the value out of the Ok variant,
    // or panic with a custom message if the result is Err
    let result = divide(2.0, 3.0).expect("Failed to divide");

    // Get the value out of the Ok variant,
    // or a default value if the result is Err
    let result = divide(2.0, 3.0).unwrap_or(f64::INFINITY);

    // Get the value out of the Ok variant,
    // or return the Err to the caller
    let result = divide(2.0, 3.0)?;

    // The ? operator above is equivalent to:
    let result = match divide(2.0, 3.0) {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    // PS: Try to avoid the panicking methods in production!

    Ok(())
}

#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
}

fn function_one() {
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Red;

    // match color {
    //     Color::Red => println!("Red"),
    //     Color::Green => println!("Green"),
    // }
    // error[E0004]: non-exhaustive patterns: `Color::Blue` not covered
}

fn function_two() {
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Red;

    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        _ => println!("Some other color"),
    }
}

fn function_three() {
    let num = 5;

    // match num {
    //     1 => println!("One"),
    //     2 => println!("Two"),
    //     3 => println!("Three"),
    // }
    // error[E0004]: non-exhaustive patterns:
    // `i32::MIN..=0_i32` and `4_i32..=i32::MAX` not covered
}

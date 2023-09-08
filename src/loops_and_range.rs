#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    // function_three();
    // function_four();
}

fn function_one() {
    let week = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    for day in week {
        println!("{day}");
    }
}

fn function_two() {
    for number in 1..5 {
        println!("{number}");
    }
    // Prints 1, 2, 3, 4

    for number in 1..=5 {
        println!("{number}");
    }
    // Prints 1, 2, 3, 4, 5
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input.trim().to_string()
}

fn function_three() {
    let mut user_input = get_user_input();
    while user_input != "quit" {
        user_input = get_user_input();
    }
}

fn function_four() {
    loop {
        let user_input = get_user_input();
        if user_input == "quit" {
            break;
        }
    }
}

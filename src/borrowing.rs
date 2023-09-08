#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
}

fn function_one() {
    // Borrowing happens through references.
    // References are immutable by default.

    fn calculate_length(s: &String) -> usize {
        // s.push_str(", world");
        // error[E0596]: cannot borrow `*s` as mutable,
        // as it is behind a `&` reference
        s.len()
    }

    let mut greeting = "hello".to_string();
    let len = calculate_length(&greeting);
    println!("The length of {greeting} is {len}");
}

fn function_two() {
    // Mutable references

    let mut greeting = "hello".to_string();
    change(&mut greeting);
    println!("The string is now {greeting}");
    // The string is now hello, world

    fn change(s: &mut String) {
        s.push_str(", world");
    }
}

fn function_three() {
    // Borrowing rules:
    // 1. At any given time, you can have either one mutable reference
    //    or any number of immutable references.
    // 2. References must always be valid.

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    // println!("{r1}, {r2}");

    // These rules prevent data races and allow "fearless concurrency"!
}

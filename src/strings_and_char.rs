#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
    function_four();
}

fn function_one() {
    // A String is a growable, mutable, owned, UTF-8 encoded type.

    let mut s1 = String::new();
    s1.push_str("hello");

    let s2 = String::from("hello");

    let s3 = "hello".to_string();

    assert_eq!(s1, s2);
    assert_eq!(s2, s3);
}

fn function_two() {
    let mut s = "hello".to_string();

    let string_slice = &s[0..4];

    println!("{string_slice}");
    // Prints "hell"
}

fn function_three() {
    /// In function signatures, `&str` is preferred over `&String` because it is more general.
    /// Look up "deref coercion" for more information.
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    let name = "Jack".to_string();
    greet(&name);
}

fn function_four() {
    // Single and double quotes mean different things!

    let this_is_a_string_slice = "x";
    let this_is_a_char = 'x';

    for char in this_is_a_string_slice.chars() {
        println!("{char}",);
    }
}

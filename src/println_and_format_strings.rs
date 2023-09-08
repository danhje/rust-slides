#![allow(dead_code, unused)]

use crate::shared::*;
use std::fmt::Formatter;

pub fn main() {
    function_one();
    function_two();
    function_three();
    function_four();
}

fn function_one() {
    // `println!` prints to the console
    // The ! indicates it is a macro (metaprogramming - code that writes code)

    println!("Hello, world!");
}

#[rustfmt::skip]
fn function_two() {
    // println! supports format strings

    let name = "Bob";
    println!("Hello, {name}");         // These
    println!("Hello, {}", name);       // are
    println!("Hello, {n}", n = name);  // equivalent
}

fn function_three() {
    let arr = [1, 2, 3];

    // The :? is sometimes needed when printing complex types
    println!("{arr:?}");
}

fn function_four() {
    // Format strings can also be interpreted by the `format!` macro

    let name = "Bob";
    let greeting = format!("Hello, {name}!");
}

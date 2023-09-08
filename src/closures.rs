#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
}

fn function_one() {
    // Closures are anonymous functions that can be stored in a variable.
    // Like lambdas in Python.

    let add_one = |x| x + 1;

    println!("{:?}", add_one(1)); // Prints "2"
}

fn function_two() {
    // Closures can capture variables from the scope
    // in which they are defined.

    let x = 2;

    // y is a parameter, x is a captured variable
    let add_x = |y| x + y;

    println!("{:?}", add_x(1)); // Prints "3"
}

fn function_three() {
    do_other_stuff();
    ()
}

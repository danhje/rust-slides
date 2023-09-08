#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
}

fn function_one() {
    // let data = Vec::new();
    // data.push(1);
    // error[E0596]: cannot borrow `data` as mutable,
    // as it is not declared as mutable
}

fn function_two() {
    let mut data = Vec::new();
    data.push(1);
}

fn function_three() {
    // Why const when we can create immutable variables with let?
    // Constants are known at compile time, meaning the compiler can optimize.

    const LOG_LEVEL: &str = "INFO";
    const NUMBERS: [i32; 3] = [1, 2, 3];

    let (first, second, third) = (1, 2, 3);
    // const MORE_NUMBERS: [i32; 3] = [first, second, third];
    // error[E0435]: attempt to use a non-constant value in a constant
}

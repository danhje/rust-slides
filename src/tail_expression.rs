#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
}

fn function_one() -> String {
    do_other_stuff();
    return "This string will be returned".to_string();
}

fn function_two() -> String {
    do_other_stuff();
    "This will be returned, even though there is no return keyword".to_string()
    // Tail expression
}

fn function_three() {
    do_other_stuff();
    "This will NOT be returned due to the trailing semicolon".to_string();
}

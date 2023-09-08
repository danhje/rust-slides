#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
}

#[rustfmt::skip]
fn function_one() {
    let num: i32 = 5;                        // With explicit type annotation

    let num = 5;                             // Rust can usually infer the type

    // let nums = (0..10).collect();            // But not always
    // error[E0282]: type annotations needed

    let nums: Vec<i32> = (0..10).collect();  // With explicit type annotation
}

fn function_two() {
    do_other_stuff();
    ()
}

fn function_three() {
    do_other_stuff();
    ()
}

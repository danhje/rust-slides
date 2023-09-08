#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
}

fn function_one() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut it = numbers.iter();

    println!("The first element is {:?}", it.next());
    println!("The second element is {:?}", it.next());
    println!("The third element is {:?}", it.next());

    // Prints:
    // The first element is Some(1)
    // The second element is Some(2)
    // The third element is Some(3)
}

fn function_two() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let negated_odd_numbers: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 1)
        .map(|&x| -x)
        .collect();
    println!("{negated_odd_numbers:?}");
    // Prints [-1, -3, -5]
}

fn function_three() {
    do_other_stuff();
    ()
}

#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
    function_four();
    function_five();
    function_six();
    function_seven();
}

fn function_one() {
    let tuple = (1, 2, 3);
    let arr = [1, 2, 3];
    let vec = vec![1, 2, 3];

    println!("tuple: {tuple:?}");
    println!("arr: {arr:?}");
    println!("vec: {vec:?}");
}

fn function_two() {
    // Tuples can store mixed types, arrays and vectors cannot

    let tuple = (1, "hello", 4.2);
    // let arr = [1, "hello", 4.2];  // error[E0308]: mismatched types
    // let vec = vec![1, "hello", 4.2];  // error[E0308]: mismatched types
}

fn function_three() {
    // Indexing

    let tuple = (1, 2, 3);
    let arr = [1, 2, 3];
    let vec = vec![1, 2, 3];

    println!("First tuple element: {}", tuple.0);
    println!("First array element: {}", arr[0]);
    println!("First vector element: {}", vec[0]);
}

fn function_four() {
    // Array length must be known at compile time,
    // and the size can't change.

    let mut arr = [1, 2, 3];
    arr = [5, 6, 7]; // This is fine
                     // arr = [1, 2];     // This is not
                     // error[E0308]: mismatched types
                     // expected an array with a fixed size of 3 elements,
                     // found one with 2 elements

    // The structural type of a tuple can't change

    let mut tup = (1, "cat");
    tup = (2, "cats"); // This is fine
                       // tup = ("many", "cats")  // This is not
                       // error[E0308]: mismatched types
                       // 59 |     tup = ("many", "cats")  // This is not
                       //    |            ^^^^^^ expected integer, found `&str`
}

fn function_five() {
    // Array length must be known at compile time

    use rand::prelude::*;
    let random_numbers = thread_rng().gen_range(1..10);
    // let unknown_number_of_elements = [0; random_numbers];  // error[E0435]: attempt to use a non-constant value in a constant
    // println!("Here's an unknown number of elements: {unknown_number_of_elements:?}");
}

fn function_six() {
    // Vec length DOES NOT have to be known at compile time

    use rand::prelude::*;

    let random_numbers = thread_rng().gen_range(1..10);
    let unknown_number_of_elements = vec![0; random_numbers];
}

fn function_seven() {
    // HashSets contain unique elements

    use std::collections::HashSet;

    let mut set = HashSet::from([1, 2, 3]);
    set.extend([2, 3, 4]);
    println!("{set:?}");
    // Prints: {1, 2, 3, 4}

    let set_one = HashSet::from([1, 2, 3]);
    let set_two = HashSet::from([3, 4, 5]);

    let in_both = set_one.intersection(&set_two);
    println!("{in_both:?}");
    // Prints: [3]

    let in_one_of_them = set_one.union(&set_two);
    println!("{in_one_of_them:?}");
    // Prints: [3, 2, 1, 5, 4]
}

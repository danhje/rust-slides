#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
}

fn function_one() {
    // A module can be defined inline.
    // Tests are a good use case for this.

    mod inline_module {
        pub fn do_something() {
            println!("Doing something");
        }
    }

    inline_module::do_something();
}

fn function_two() {
    // Separate files are also modules,
    // and can be brought in with the same
    // `mod` keyword.

    // mod separate_file_module; // This only works from main.rs or lib.rs

    separate_file_module::do_something();

    use crate::separate_file_module; // This works from anywhere

    separate_file_module::do_something();
}

fn function_three() {
    // mod vs. use
    // While `mod` brings the contents of a module into scope,
    // the full path must still be used to access the contents.
    // `use` can be used to bind a full path to a new name, for easier access.

    let hm1 = std::collections::HashSet::from([1, 2, 3]);

    use std::collections::HashSet;
    let hm2 = HashSet::from([1, 2, 3]);

    use std::collections::HashSet as HS;
    let hm3 = HS::from([1, 2, 3]);
}

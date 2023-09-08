#![allow(dead_code, unused)]

pub fn main() {
    first();
    second();
}

pub fn first() {
    // Ownership rules:
    // 1. Each value in Rust has a variable that is its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let s: String = "hello".to_string();
    let new_s: String = s;
    // println!("The string is {s}");
    // error[E0382]: borrow of moved value: `s`
}

pub fn second() {
    // Ownership rules:
    // 1. Each value in Rust has a variable that is its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    fn takes_ownership(new_string_owner: String) {
        // The string now belongs to the variable new_string_owner.
        // The string isn't returned, therefore it is simply dropped when
        // new_string_owner goes out of scope at the end of this function.
    }

    let s = "hello".to_string();
    // takes_ownership(s);
    println!("The string is {s}");
    // error[E0382]: borrow of moved value: `s`
}

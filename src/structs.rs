#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
}

fn function_one() {
    struct PowerStation {
        name: String,
        capacity: f64,
    }

    let kvildal = PowerStation {
        name: "Kvildal".to_string(),
        capacity: 1240.0,
    };

    let tonstad = PowerStation {
        name: "Tonstad".to_string(),
        capacity: 960.0,
    };
}

fn function_two() {
    // A tuple struct is a struct that has unnamed fields.

    struct Point(f64, f64);

    let origin = Point(0.0, 0.0);
    let point = Point(3.0, 4.0);
}

fn function_three() {
    // A unit struct is a struct that has no fields.

    struct NothingToSeeHere;

    let nope = NothingToSeeHere;
}

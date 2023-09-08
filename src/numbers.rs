#![allow(dead_code, unused)]

pub fn main() {
    function_one();
    function_two();
    function_three();
    function_four();
    function_five();
}

fn function_one() {

    //   Length	    Signed	Unsigned
    //   8-bit	    i8	    u8
    //   16-bit	    i16	    u16
    //   32-bit	    i32	    u32
    //   64-bit	    i64	    u64
    //   128-bit	i128	u128
    //   arch	    isize	usize
}

fn function_two() {
    // Ints defaults to i32

    let num = 42; // This is an i32

    // If you want a different type:

    let num_1: u8 = 42;
    let num_2 = 42 as u8;
    let num_3 = 42u8;
    let num_4 = 42_u8;

    assert!(num_1 == num_2 && num_2 == num_3 && num_3 == num_4);
}

fn function_three() {
    // Be careful when narrowing!

    let negative = -5_i8;
    let positive = negative as u8;
    println!("{positive}");
    // Prints 251
}

fn function_four() {
    use std::convert::TryFrom;

    let negative: i8 = -5;
    let positive = u8::try_from(negative);

    match positive {
        Ok(val) => println!("Successfully converted to u8: {}", val),
        Err(_) => println!("Failed to convert to u8"),
    }
}

fn function_five() {
    // Floats only have two types: f32 and f64
    // f64 is the default

    let num = 42.0;
}

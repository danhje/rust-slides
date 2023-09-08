#![allow(dead_code, unused)]

pub fn main() {
    function_one();
    function_two();
    function_three();
}

fn function_one() {
    // Literal patterns

    let number = 3;

    match number {
        1 => println!("All is one"),
        2 => println!("Two's company"),
        3 => println!("Three's a crowd"),
        _ => println!("Undefined territory"),
    }
}

fn function_two() {
    // Patterns can be used with `let`, `if let`, `for` loops,
    // function parameters, etc.

    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    let colors = [
        Color {
            r: 65,
            g: 250,
            b: 9,
        },
        Color {
            r: 8,
            g: 164,
            b: 18,
        },
        Color {
            r: 241,
            g: 9,
            b: 98,
        },
    ];

    for Color { r, g, b } in colors {
        println!("{r}-{g}-{b}");
    }
}

fn function_three() {
    // Refutability: Patterns come in two forms, refutable and irrefutable.

    // The pattern (x, y) is irrefutable, i.e. it will allways match:

    let point = (4, 3);
    let (x, y) = point;

    // The pattern Some(x) is refutable, i.e. it might not match:

    let maybe_a_number = Some(3);

    if let Some(n) = maybe_a_number {
        println!("It's something");
    } else {
        println!("There's nothing there");
    };
}

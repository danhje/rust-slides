#![allow(dead_code, unused)]
use crate::shared::*;
use std::collections::HashSet;

pub fn main() {
    function_one();
    function_two();
    function_three();
    function_four();
}

fn function_one() {
    // In Rust, almost everything is an expression,
    // meaning it evaluates to a value.
    // For example, `if` is an expression, not a statement.

    let answer = if 1 + 1 == 2 {
        do_stuff();
        42
    } else {
        do_other_stuff();
        24
    };

    println!("The answer is {answer}") // Prints "The answer is 42"
}

fn function_two() {
    // `match` is also an expression.

    let answer = match 1 + 1 {
        2 => 42,
        _ => 24,
    };

    println!("The answer is {answer}") // Prints "The answer is 42"
}

#[rustfmt::skip]
fn function_three() {
    let answer = if 1 + 1 == 2 {
        match 1 + 1 {
            2 => {
                {
                    {
                        {
                            42
                        }
                    }
                }
            }
            _ => 24
        }
    }
    else {
        24
    };

    println!("The answer is {answer}")  // Prints "The answer is 42"
}

fn function_four() {
    let x: u8 = 2;
    let y: u8 = 3;
    let rival_team: &HashSet<(u8, u8)> = &HashSet::new();

    // Everything is an expression, including block.
    // How can this be used? Might help to communicate intent. Compare these two, wo are doing the exact same thing:

    // As we are reading the first line below, the intent isn't necessarily immediately clear:
    let mut moves = HashSet::from([(x + 1, y + 1)]);
    if let Some(new_x) = x.checked_sub(1) {
        moves.insert((new_x, y + 1));
    }
    let capture_moves = moves
        .intersection(rival_team)
        .cloned()
        .collect::<HashSet<_>>();

    // But here, we immediately understand that all the code in the block is involved in computing capture_moves:
    let capture_moves = {
        let mut moves = HashSet::from([(x + 1, y + 1)]);
        if let Some(new_x) = x.checked_sub(1) {
            moves.insert((new_x, y + 1));
        }
        moves
            .intersection(rival_team)
            .cloned()
            .collect::<HashSet<_>>()
    };

    // An additional advantage of the latter is that the temporary variable `moves` is contained in the scope
    // (it's cleared after the block has finished)
}

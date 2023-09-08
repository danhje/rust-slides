#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
}

fn function_one() {
    use std::ops::Add;

    #[derive(Debug)]
    struct User {
        name: String,
    }

    #[derive(Debug)]
    struct Group {
        users: Vec<User>,
    }

    impl Add for User {
        type Output = Group;

        fn add(self, other: User) -> Group {
            Group {
                users: vec![self, other],
            }
        }
    }

    let user1 = User {
        name: String::from("Alice"),
    };
    let user2 = User {
        name: String::from("Bob"),
    };

    let group = user1 + user2;

    println!("{group:#?}");
    // Group {
    //     users: [
    //         User {
    //             name: "Alice",
    //         },
    //         User {
    //             name: "Bob",
    //         },
    //     ],
    // }
}

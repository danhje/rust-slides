use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
}

fn function_one() {
    enum Color {
        Red,
        Green,
        Blue,
    }

    let favorite_color = Color::Blue;
}

fn function_two() {
    enum Pet {
        Goldfish,
        Cat(String),
        Dog { name: String, age: u8 },
    }

    let first_pet = Pet::Goldfish;
    let second_pet = Pet::Cat("Misty".to_string());
    let third_pet = Pet::Dog {
        name: "Rusty".to_string(),
        age: 8,
    };
}

fn function_three() {
    do_other_stuff();
    ()
}

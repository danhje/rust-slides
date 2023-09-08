#![allow(dead_code, unused)]
use crate::shared::*;

pub fn main() {
    function_one();
    function_two();
    function_three();
    function_four()
}

fn function_one() {
    struct Dog {
        name: String,
    }

    impl Dog {
        fn bark(&self) {
            println!("{} barks", self.name);
        }
    }

    let fido = Dog {
        name: "Fido".to_string(),
    };

    fido.bark();
}

fn function_two() {
    trait Animal {
        fn speak(&self);
    }

    struct Dog;
    struct Cat;
    struct Fox;

    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }

    impl Animal for Cat {
        fn speak(&self) {
            println!("Meow!");
        }
    }

    impl Animal for Fox {
        fn speak(&self) {
            println!("Ring-ding-ding-ding-dingeringeding!");
        }
    }
}

fn function_three() {
    trait Animal {
        fn speak(&self);
    }

    struct Dog;

    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }

    fn make_animal_speak(animal: &impl Animal) {
        // This function accepts any type that implements the Animal trait.
        animal.speak();
    }

    let dog = Dog;
    make_animal_speak(&dog);
}

fn function_four() {
    struct Dog;

    impl Dog {
        fn speak() {
            // Note that `speak` doesn't have a &self parameter,
            // meaning it becomes an "associated function" (think static method)
            println!("Woof!");
        }
    }

    // Dog.speak();  // error[E0599]: no method named `speak` found for struct
    Dog::speak(); // :: is used when accessing associated functions
}

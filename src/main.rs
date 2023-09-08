mod _template;
mod borrowing;
mod closures;
mod containers;
mod enums;
mod everything_is_an_expression;
mod impl_traits;
mod iterators;
mod loops_and_range;
mod match_exhaustive;
mod mod_and_use;
mod mutability;
mod numbers;
mod operator_overloading;
mod option_result;
mod ownership;
mod patterns;
mod println_and_format_strings;
mod separate_file_module;
mod shared;
mod strings_and_char;
mod structs;
mod tail_expression;
mod type_annotation;

fn main() {
    _template::main();
    println_and_format_strings::main();
    type_annotation::main();
    numbers::main();
    strings_and_char::main();
    mutability::main();
    structs::main();
    enums::main();
    option_result::main();

    containers::main();
    loops_and_range::main();
    closures::main();
    iterators::main();

    ownership::main();
    borrowing::main();

    patterns::main();
    match_exhaustive::main();

    tail_expression::main();
    everything_is_an_expression::main();
    impl_traits::main();

    mod_and_use::main();
    // operator_overloading::main();
}

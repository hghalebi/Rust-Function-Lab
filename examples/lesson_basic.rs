use rust_functions_learning::lessons::lesson_core as core;

fn main() {
    // Start by observing the contract: no input and immutable output.
    println!("{}", core::say_hello());
    // Borrowed input lets us reuse string data without ownership moves.
    println!("{}", core::greet("Ada"));
    // `add` is the smallest pure example of a typed function unit.
    println!("{}", core::add(2, 3));
}

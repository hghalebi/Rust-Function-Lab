use rust_functions_learning::lessons::ownership::{Counter, User, borrow_name};

fn main() {
    // Borrowing avoids moving the text; this line proves the function can inspect
    // input without owning it.
    println!("Borrowed length: {}", borrow_name("Rust"));

    let mut counter = Counter::new();
    // `mut` is needed because `increment` mutates internal state through
    // `&mut self`.
    counter.increment();

    let user = User {
        name: "Hamze".to_string(),
    };
    // Method form (`user.greet()`) shows where `self` is borrowed by default in
    // read-only receiver APIs.
    println!("{}", user.greet());
    // Confirm the counter state is now `1`.
    println!("Counter value: {}", counter.get());
}

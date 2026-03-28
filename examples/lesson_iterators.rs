use rust_functions_learning::lessons::iterators::{even_times_ten, max_even_tripled};

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    // A chained iterator pipeline: keep evens, transform them, and materialize.
    println!("Even*10: {:?}", even_times_ten(&numbers));
    // Demonstrates `into_iter` ownership transfer + consume-through `max`.
    println!("Max even tripled: {:?}", max_even_tripled(numbers));
}

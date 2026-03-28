use rust_functions_learning::lessons::memoization::memoized_square;

fn main() {
    let mut memo = memoized_square();
    // First call computes and stores in closure-owned cache.
    println!("{}", memo(4));
    // Second call with same input should reuse cache.
    println!("{}", memo(4));
}

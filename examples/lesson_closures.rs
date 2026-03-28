use rust_functions_learning::lessons::closures::{apply_twice, make_adder};

fn main() {
    // Closure literals are first-class values, and they can be passed as arguments.
    let plus_two = |x| x + 2;
    println!("apply twice: {}", apply_twice(plus_two, 5));

    // `make_adder` returns a specialized closure that captures `10` in its own state.
    let add_ten = make_adder(10);
    println!("adder: {}", add_ten(3));
}

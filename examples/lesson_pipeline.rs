use rust_functions_learning::lessons::pipeline::process;

fn main() {
    // Observe how parse, validate, compute, and collect are staged in one expression.
    let result = process(vec!["4", "7", "4"]);
    println!("{result:?}");
}

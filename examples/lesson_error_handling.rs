use rust_functions_learning::lessons::error_handling::half_of_positive;

fn main() {
    // A happy path value returns `Ok`; this keeps data moving through the pipeline.
    println!("{:?}", half_of_positive("42"));
    // Invalid business rule demonstrates early return through `?` and `Err`.
    println!("{:?}", half_of_positive("-1"));
}

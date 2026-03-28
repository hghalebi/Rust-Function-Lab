//! Basic function signatures, methods, and function-like composition.
//! 
//! This file maps the first half of the learning ladder: small, pure-ish functions
//! that do exactly one thing and communicate expectations through signatures.

/// Section 1: simplest function.
pub fn say_hello() -> &'static str {
    // A static string literal never gets deallocated and never moves, so its
    // return type is guaranteed to be valid for the full lifetime of the program.
    "Hello"
}

/// Section 2: function with input.
pub fn greet(name: &str) -> String {
    // Borrowing the input lets callers pass owned or borrowed text without forcing
    // an allocation or ownership transfer in the call path.
    format!("Hello {name}")
}

/// Section 3: function with output.
pub fn add(a: i32, b: i32) -> i32 {
    // Keep the return type explicit so this remains a typed mapping from two i32 inputs
    // to a single i32 output.
    a + b
}

/// Section 12: passing behavior as input.
pub fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    // Passing behavior as an argument is what makes composition possible; `apply`
    // itself remains tiny and the transformation is injected by the caller.
    f(value)
}

/// Section 27: explicit function composition.
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    // `compose` builds a new callable that always evaluates `g` first, then `f`.
    // This keeps pipelines explicit and avoids duplicating control flow at call sites.
    move |x| f(g(x))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Section 26: destructuring in function parameters.
pub fn point_sum(Point { x, y }: Point) -> i32 {
    // Destructuring in the signature makes the function contract read like a
    // data shape, not a sequence of field-extraction statements.
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn say_hello_returns_expected_text() {
        assert_eq!(say_hello(), "Hello");
    }

    #[test]
    fn greet_uses_input_name() {
        assert_eq!(greet("Hamze"), "Hello Hamze");
    }

    #[test]
    fn add_sums_two_numbers() {
        assert_eq!(add(3, 4), 7);
    }

    #[test]
    fn apply_calls_closure() {
        let double = |x| x * 2;
        assert_eq!(apply(double, 21), 42);
    }

    #[test]
    fn compose_mul_then_add() {
        let double = |x| x * 2;
        let add_five = |x| x + 5;
        let composed = compose(double, add_five);
        assert_eq!(composed(3), 16);
    }

    #[test]
    fn point_sum_adds_components() {
        let point = Point { x: 4, y: 7 };
        assert_eq!(point_sum(point), 11);
    }
}

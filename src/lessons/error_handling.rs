//! Option, Result, and ergonomic fallible composition.
//!
//! The design rule is to make failure explicit in the return type while keeping
//! successful flow linear and readable.

pub fn parse_number(s: &str) -> Result<i32, String> {
    // Parse errors are converted to a caller-facing string so the call site always
    // handles either success or actionable failure.
    s.parse::<i32>()
        .map_err(|_| format!("Could not parse `{s}`"))
}

pub fn require_positive(n: i32) -> Result<i32, String> {
    // Domain validation belongs in a small function, so invalid inputs fail close to
    // where the business rule is defined.
    if n > 0 {
        Ok(n)
    } else {
        Err("Number must be positive".to_string())
    }
}

pub fn half_of_positive(s: &str) -> Result<i32, String> {
    // `?` keeps the pipeline flat: each step either produces `Ok(value)` or exits
    // early with the exact `Err` payload.
    let n = parse_number(s)?;
    let positive = require_positive(n)?;
    Ok(positive / 2)
}

pub fn parse_map_twice(s: &str) -> Result<i32, String> {
    // `map` changes only the success branch and keeps error propagation unchanged.
    parse_number(s).map(|n| n * 2)
}

pub fn result_chain_with_and_then(s: &str) -> Result<i32, String> {
    // `and_then` is used when the next function also returns `Result`, because it
    // flattens the nested value automatically.
    parse_number(s).and_then(require_positive)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_ok_for_digits() {
        assert_eq!(parse_number("7"), Ok(7));
    }

    #[test]
    fn parse_number_err_for_text() {
        assert!(parse_number("abc").is_err());
    }

    #[test]
    fn require_positive_filters_negative() {
        assert!(require_positive(-2).is_err());
        assert_eq!(require_positive(4), Ok(4));
    }

    #[test]
    fn half_of_positive_uses_question_mark() {
        assert_eq!(half_of_positive("10"), Ok(5));
        assert!(half_of_positive("-10").is_err());
        assert!(half_of_positive("oops").is_err());
    }

    #[test]
    fn parse_map_and_chain_are_predictable() {
        assert_eq!(parse_map_twice("9"), Ok(18));
        assert_eq!(result_chain_with_and_then("8"), Ok(8));
        assert!(result_chain_with_and_then("0").is_err());
    }
}

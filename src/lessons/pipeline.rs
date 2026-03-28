//! End-to-end pipeline combining parsing, validation, transformation, and memoization.
//! 
//! This module demonstrates production shape: small tested units connected with an
//! explicit error path and a shared compute strategy.

use std::collections::HashMap;

use crate::lessons::error_handling::{parse_number, require_positive};

pub fn compute(n: i32) -> i32 {
    // Single-purpose pure transform that stays deterministic so caching and reuse
    // stay trustworthy.
    n * 10
}

pub fn make_memoized_compute() -> impl FnMut(i32) -> i32 {
    let mut cache = HashMap::new();

    move |n| *cache.entry(n).or_insert_with(|| compute(n))
}

pub fn process(inputs: Vec<&str>) -> Result<Vec<i32>, String> {
    let mut memo = make_memoized_compute();
    // This pipeline is deterministic: parse -> validate -> transform. If parse or
    // validation fails at any element, work stops and the error is returned.
    inputs
        .into_iter()
        .map(|value| {
            let n = parse_number(value)?;
            let n = require_positive(n)?;
            Ok(memo(n))
        })
        .collect()
}

pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    // The lifetime ties the output to whichever input reference is returned; this
    // prevents returning references that might outlive both arguments.
    if a.len() > b.len() { a } else { b }
}

pub trait Processor {
    fn process(&self, input: i32) -> i32;
}

pub struct Double;

impl Processor for Double {
    fn process(&self, input: i32) -> i32 {
        // This trait object keeps processors swappable in runtime order without
        // changing the pipeline driver.
        input * 2
    }
}

pub struct AddFive;

impl Processor for AddFive {
    fn process(&self, input: i32) -> i32 {
        // A second implementation demonstrates behavioral composition through shared
        // trait abstraction rather than branching.
        input + 5
    }
}

pub fn run_pipeline(processors: &[Box<dyn Processor>], mut value: i32) -> i32 {
    // Passing boxed trait objects here makes extensibility a runtime decision while
    // preserving a fixed orchestration loop.
    for p in processors {
        value = p.process(value);
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_parses_valid_inputs_and_memoizes() {
        assert_eq!(process(vec!["4", "7", "4"]), Ok(vec![40, 70, 40]));
    }

    #[test]
    fn pipeline_rejects_negative_input() {
        assert!(process(vec!["4", "-1", "10"]).is_err());
    }

    #[test]
    fn pipeline_rejects_invalid_input() {
        assert!(process(vec!["4", "abc", "10"]).is_err());
    }

    #[test]
    fn lifetime_example_picks_longer_reference() {
        let a = "short";
        let b = "longer";
        assert_eq!(longest(a, b), "longer");
    }

    #[test]
    fn trait_object_pipeline_runs_in_order() {
        let processors: Vec<Box<dyn Processor>> = vec![Box::new(Double), Box::new(AddFive)];
        assert_eq!(run_pipeline(&processors, 4), 13);
    }
}

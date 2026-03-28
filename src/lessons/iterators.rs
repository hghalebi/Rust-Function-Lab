//! Iterator fundamentals and functional-style combinators.
//!
//! Each method builds a tiny transformation stage. The important pattern is that
//! none of these stages execute immediately without a terminal consumer.

pub fn even_times_ten(numbers: &[i32]) -> Vec<i32> {
    // `iter()` borrows the input slice, so we can keep the caller-owned collection
    // after transformation.
    numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| *x * 10)
        .collect()
}

pub fn lengths(names: &[String]) -> Vec<usize> {
    // Keeping references during `iter()` makes this stage reusable in pipelines
    // without consuming `names`.
    names.iter().map(|name| name.len()).collect()
}

pub fn parse_numbers(inputs: &[&str]) -> Vec<i32> {
    // `filter_map` is the practical parser style: parse only what is valid and drop
    // everything else in one pass.
    inputs
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

pub fn chars_from_words(words: &[&str]) -> Vec<char> {
    // `flat_map` expands each token into many output values (one string to many
    // chars), while flattening the nested iterator structure for the caller.
    words.iter().flat_map(|word| word.chars()).collect()
}

pub fn fold_sum(numbers: &[i32]) -> i32 {
    // `copied` converts `&i32` to `i32`, then `sum` reduces the sequence into one
    // accumulator.
    numbers.iter().copied().sum()
}

pub fn max_even_tripled(numbers: Vec<i32>) -> Option<i32> {
    // Ownership of `numbers` is consumed deliberately, because this function returns
    // only the derived result and does not need the original vector afterwards.
    numbers
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 3)
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_only_even_and_scale() {
        assert_eq!(even_times_ten(&[1, 2, 7, 10]), vec![20, 100]);
    }

    #[test]
    fn map_lengths_without_consuming_input() {
        let names = vec!["Ada".to_string(), "Grace".to_string()];
        assert_eq!(lengths(&names), vec![3, 5]);
    }

    #[test]
    fn filter_map_parses_numbers() {
        assert_eq!(parse_numbers(&["1", "abc", "42"]), vec![1, 42]);
    }

    #[test]
    fn flat_map_expands_words_to_chars() {
        assert_eq!(
            chars_from_words(&["rust", "go"]),
            vec!['r', 'u', 's', 't', 'g', 'o']
        );
    }

    #[test]
    fn fold_accumulates_values() {
        assert_eq!(fold_sum(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn process_pipeline_returns_max_even_times_ten() {
        assert_eq!(max_even_tripled(vec![1, 4, 7, 10]), Some(30));
        assert_eq!(max_even_tripled(vec![1, 3]), None);
    }
}

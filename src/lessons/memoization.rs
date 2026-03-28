//! Memoization patterns: single-threaded and shared-state variants.
//!
//! Memoization is about changing computation shape: from recompute-on-demand to
//! cache-aware behavior with ownership-bound state.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

pub fn memoized_square() -> impl FnMut(i32) -> i32 {
    let mut cache = HashMap::new();

    move |x| {
        // `entry` lets one branch read the cached value and the other branch compute
        // and store it with one coherent path.
        *cache.entry(x).or_insert_with(|| {
            println!("computing...");
            x * x
        })
    }
}

pub fn memoize<F, A, R>(mut f: F) -> impl FnMut(A) -> R
where
    F: FnMut(A) -> R,
    A: Eq + Hash + Clone,
    R: Clone,
{
    let mut cache: HashMap<A, R> = HashMap::new();

    move |arg: A| {
        // Cache lookup is attempted first so repeated work is skipped whenever
        // the exact argument was seen before.
        if let Some(value) = cache.get(&arg) {
            return value.clone();
        }

        // Clone input/output to satisfy move semantics while preserving one computed
        // value for both caller and cache.
        let result = f(arg.clone());
        cache.insert(arg, result.clone());
        result
    }
}

pub fn memoized_square_threadsafe() -> impl Fn(i32) -> i32 {
    let cache = Arc::new(Mutex::new(HashMap::new()));

    move |x| {
        // `Arc<Mutex<_>>` gives shared ownership and safe mutation across closures.
        let mut map = cache.lock().unwrap_or_else(|err| err.into_inner());
        if let Some(v) = map.get(&x) {
            return *v;
        }

        // The expensive branch is hidden behind the mutex lock, so state mutation and
        // reads remain serialized and consistent.
        let value = x * x;
        map.insert(x, value);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memoized_square_reuses_cached_values() {
        let mut f = memoized_square();
        assert_eq!(f(4), 16);
        assert_eq!(f(4), 16);
        assert_eq!(f(5), 25);
    }

    #[test]
    fn memoize_generic_cache_reuses_values() {
        let mut square = memoize(|x: i32| {
            println!("compute");
            x * x
        });

        assert_eq!(square(6), 36);
        assert_eq!(square(6), 36);
    }

    #[test]
    fn memoized_square_threadsafe_is_reusable() {
        let f = memoized_square_threadsafe();
        assert_eq!(f(3), 9);
        assert_eq!(f(3), 9);
    }
}

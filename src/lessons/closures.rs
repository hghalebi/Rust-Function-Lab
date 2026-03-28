//! Closure behavior, traits, and stateful callables.
//! 
//! Closures are where behavior becomes data: every closure carries both code and the
//! context needed to execute it.

/// Plain function pointer version.
pub fn double(x: i32) -> i32 {
    // A plain function pointer remains the simplest behavior unit when no captured
    // environment is required.
    x * 2
}

pub fn apply_pointer(f: fn(i32) -> i32, value: i32) -> i32 {
    // This signature accepts only free functions and non-capturing closures.
    f(value)
}

pub fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    // `f` is reused on the intermediate output to show pure behavior composition.
    f(f(x))
}

pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // `move` keeps `n` inside the returned callable so callers can pass the
    // builder away without lifetime ties to local stack frames.
    move |x| x + n
}

pub fn closure_with_backpack(x: i32, bonus: i32) -> i32 {
    // This closure captures `bonus` from scope, demonstrating "read-only state"
    // carried with behavior.
    let add_bonus = |input| input + bonus;
    add_bonus(x)
}

pub fn stateful_counter() -> impl FnMut() -> usize {
    let mut count = 0usize;
    move || {
        // The returned closure is `FnMut` because it updates captured `count`
        // every time it is invoked.
        count += 1;
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointer_based_function_is_accepted() {
        assert_eq!(apply_pointer(double, 4), 8);
    }

    #[test]
    fn apply_twice_calls_function_two_times() {
        let plus_one = |x| x + 1;
        assert_eq!(apply_twice(plus_one, 5), 7);
    }

    #[test]
    fn adder_factory_adds_n() {
        let add_five = make_adder(5);
        assert_eq!(add_five(3), 8);
    }

    #[test]
    fn closure_captures_bonus() {
        assert_eq!(closure_with_backpack(6, 4), 10);
    }

    #[test]
    fn stateful_counter_mutates_internal_state() {
        let mut counter = stateful_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }
}

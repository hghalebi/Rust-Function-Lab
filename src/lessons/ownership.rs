//! Borrowing, ownership, and methods.
//! 
//! Ownership questions are architectural questions here: where do values live, who
//! is allowed to mutate them, and when can they be reused by later steps.

/// Demonstrates borrowed string usage.
pub fn borrow_name(name: &str) -> usize {
    // Borrowing (`&str`) makes this function a read-only observer and keeps caller
    // ownership stable for subsequent code.
    name.len()
}

/// Demonstrates owned `String` usage.
pub fn take_name(name: String) -> String {
    // Taking ownership gives this function full control of the string and allows
    // it to transform and return ownership intentionally.
    name.to_ascii_uppercase()
}

#[derive(Debug)]
pub struct User {
    pub name: String,
}

impl User {
    /// Method with immutable borrow.
    pub fn greet(&self) -> String {
        // `&self` signals this method is side-effect free from the perspective of
        // the `User` instance.
        format!("Hello {}", self.name)
    }
}

#[derive(Debug)]
pub struct Counter {
    pub value: i32,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn get(&self) -> i32 {
        // Immutable read path so callers can inspect state without changing it.
        self.value
    }

    pub fn increment(&mut self) {
        // `&mut self` is required because this method mutates internal state.
        self.value += 1;
    }

    pub fn into_value(self) -> i32 {
        // Consuming `self` makes ownership transfer explicit at the API boundary.
        self.value
    }
}

/// Section 24: pattern-based control flow via match.
pub fn describe(value: i32) -> &'static str {
    // `match` encodes domain partitions directly in control flow. The compiler
    // forces all branches to be represented, which is safer than scattered if/else.
    match value {
        n if n > 0 => "positive",
        0 => "zero",
        _ => "negative",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_name_measures_len_without_move() {
        let raw = String::from("Hamze");
        let len = borrow_name(&raw);
        assert_eq!(len, 5);
        assert_eq!(raw, "Hamze");
    }

    #[test]
    fn take_name_consumes_and_returns_owned_result() {
        let name = String::from("rust");
        let out = take_name(name);
        assert_eq!(out, "RUST");
    }

    #[test]
    fn counter_increment_changes_state() {
        let mut counter = Counter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
        assert_eq!(counter.into_value(), 2);
    }

    #[test]
    fn user_greet_borrows_self() {
        let user = User {
            name: "Ada".to_string(),
        };
        assert_eq!(user.greet(), "Hello Ada");
    }

    #[test]
    fn describe_is_deterministic() {
        assert_eq!(describe(3), "positive");
        assert_eq!(describe(0), "zero");
        assert_eq!(describe(-1), "negative");
    }
}

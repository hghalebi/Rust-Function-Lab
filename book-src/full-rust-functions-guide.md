# Functions in Rust
## From Basic Syntax to Typed Contracts, Tests, and Agentic Pipelines

Rust looks simple at first.

You may see a function like this:

```rust
fn square(x: i32) -> i32 {
    x * x
}
```

And think:

"Okay. That is just a function."

But Rust quietly puts a lot of meaning into that one line.

* `fn` says this is a function
* `square` is the function name
* `x: i32` says the input is a 32-bit integer
* `-> i32` says the output is also a 32-bit integer
* `x * x` is the returned value

So even the simplest Rust function is already a contract:

> Give me this kind of input, and I promise this kind of output.

That is the first important mindset shift.

A function is not just some code.
A function is a promise with types.

---

If you want runnable code references, jump to:

- [Examples and Scripts](./examples-and-scripts.md)
- [Lesson Map](./lesson-map.md)
- [Quality Gates](./quality-gates.md)

---

# 1. The Simplest Possible Function

Let us start with a function that takes no input and returns nothing.

```rust
fn say_hello() {
    println!("Hello");
}

fn main() {
    say_hello();
}
```

## What is happening?

* `fn say_hello()` defines a function named `say_hello`
* `()` means it takes no input
* `{ ... }` contains the function body
* `main()` is the program entry point
* when the program starts, Rust runs `main`
* inside `main`, `say_hello();` calls the function

## Mental model

Think of a function like a coffee machine button.

You press the button. Something happens.

You do not need to think about how the inside works every time you use it.

That is why functions exist: they let you hide details behind a clean interface.

## Unit test

This function prints text, which is a side effect.
That makes it harder to test directly.

So for teaching, it is often better to separate pure logic from printing.

```rust
fn hello_message() -> &'static str {
    "Hello"
}

fn say_hello() {
    println!("{}", hello_message());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_message_returns_hello() {
        assert_eq!(hello_message(), "Hello");
    }
}
```

## Real use case

In production systems, this pattern matters a lot:

* one function builds the message
* another function sends or prints it

That makes the logic easier to test.

---

# 2. Functions With Input

Now let us make the function more useful.

```rust
fn greet(name: &str) {
    println!("Hello {}", name);
}

fn main() {
    greet("Hamze");
}
```

## What changed?

Now the function has an input:

```rust
name: &str
```

This means:

* the parameter is named `name`
* its type is `&str`

`&str` means borrowed text, also called a string slice.

So `greet` does not take ownership of the text.
It only reads it.

## Better testable version

```rust
fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

fn greet(name: &str) {
    println!("{}", greeting(name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_builds_expected_message() {
        assert_eq!(greeting("Hamze"), "Hello Hamze");
    }

    #[test]
    fn greeting_works_with_other_names() {
        assert_eq!(greeting("Ada"), "Hello Ada");
    }
}
```

## Real use case

A chatbot, assistant, or onboarding service often has this shape:

* input: user name
* function builds message
* another layer displays or sends it

That separation is small, but very important.

---

# 3. Functions With Output

Now let us make a function that returns a value.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(2, 3);
    println!("{}", result);
}
```

## Read the signature carefully

```rust
fn add(a: i32, b: i32) -> i32
```

This says:

* input 1: `a`, an `i32`
* input 2: `b`, an `i32`
* output: an `i32`

## Unit tests

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_negative_and_positive() {
        assert_eq!(add(-2, 5), 3);
    }

    #[test]
    fn add_zero() {
        assert_eq!(add(10, 0), 10);
    }
}
```

## Real use case

Many production systems have tiny pure functions like this at the center:

* scoring
* ranking
* cost calculation
* metrics aggregation
* feature transformation

Small pure functions are the atoms of reliable systems.

---

# 4. Expressions vs Statements

This line works:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

But this fails:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b; // wrong here
}
```

## Why?

Because `a + b;` is now a statement, not the final value of the block.

A statement here evaluates to `()`.

So Rust sees:

* you promised `i32`
* but your function body ends with `()`

## Correct mental model

* final expression without `;` -> becomes the return value
* semicolon -> discard the value
* `return expr;` -> explicit return

## Teaching test

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_expression_becomes_return_value() {
        assert_eq!(add(4, 6), 10);
    }
}
```

---

# 5. Borrowing vs Owning in Function Parameters

Compare these two:

```rust
fn borrow_name(name: &str) {
    println!("Borrowed: {}", name);
}

fn take_name(name: String) {
    println!("Owned: {}", name);
}
```

## What is the difference?

* `&str` means the function borrows text
* `String` means the function owns the value

## Example

```rust
fn borrow_name(name: &str) -> String {
    format!("Borrowed: {}", name)
}

fn take_name(name: String) -> String {
    format!("Owned: {}", name)
}

fn main() {
    let name = String::from("Hamze");

    println!("{}", borrow_name(&name));
    println!("{}", name); // still usable

    println!("{}", take_name(name));
    // println!("{}", name); // not usable anymore
}
```

## Unit tests

```rust
fn borrow_name(name: &str) -> String {
    format!("Borrowed: {}", name)
}

fn take_name(name: String) -> String {
    format!("Owned: {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_name_does_not_require_ownership() {
        let name = String::from("Hamze");
        let result = borrow_name(&name);

        assert_eq!(result, "Borrowed: Hamze");
        assert_eq!(name, "Hamze");
    }

    #[test]
    fn take_name_consumes_owned_string() {
        let name = String::from("Ada");
        let result = take_name(name);

        assert_eq!(result, "Owned: Ada");
    }
}
```

## Real use case

This matters constantly in agent systems:

* borrow when reading prompts, documents, or config
* take ownership when storing messages, tasks, or outputs

That is not just syntax.
That is architecture.

---

# 6. Function Signatures Are Contracts

A Rust function signature tells you:

* what goes in
* what comes out
* whether data is borrowed or owned
* whether failure is possible

Example:

```rust
fn parse_number(s: &str) -> Result<i32, String>
```

This means:

* input: borrowed text
* output:
  * `Ok(i32)` on success
  * `Err(String)` on failure

## Example implementation with tests

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.trim()
        .parse::<i32>()
        .map_err(|_| format!("Could not parse `{s}`"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_success() {
        assert_eq!(parse_number("42"), Ok(42));
    }

    #[test]
    fn parse_number_trims_whitespace() {
        assert_eq!(parse_number(" 7 "), Ok(7));
    }

    #[test]
    fn parse_number_failure() {
        assert!(parse_number("abc").is_err());
    }
}
```

## Agentic use case

In a Rig-style extraction pipeline, a function like this is a contract boundary.

For example:

* model output arrives as text
* parser function turns it into typed data
* invalid output becomes `Err(...)`
* the agent can retry, repair, or escalate

That is how you stop random model output from turning into random system behavior.

---

# 7. Methods: Functions Attached to Data

Sometimes a function belongs to a type.

That is a method.

```rust
struct User {
    name: String,
}

impl User {
    fn greet(&self) -> String {
        format!("Hello {}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_greet_returns_expected_message() {
        let user = User {
            name: "Hamze".to_string(),
        };

        assert_eq!(user.greet(), "Hello Hamze");
    }
}
```

## Why methods matter

Methods let you attach behavior to data.

That gives you cleaner domain models.

## Real use case

In a production agent system, you may have domain types like:

* `Prompt`
* `ToolCall`
* `Task`
* `AgentState`
* `Evidence`

Each can have methods that express valid behavior clearly.

---

# 8. Pure Functions and Side Effects

A pure function:

* same input -> same output
* no side effects

```rust
fn square(x: i32) -> i32 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_of_4_is_16() {
        assert_eq!(square(4), 16);
    }

    #[test]
    fn square_of_negative_is_positive() {
        assert_eq!(square(-3), 9);
    }
}
```

## Side-effect version

```rust
fn log_and_square(x: i32) -> i32 {
    println!("computing...");
    x * x
}
```

## Real use case

Best production pattern:

> pure logic in the center, side effects at the edges

For agents, that often means:

* pure functions: validation, parsing, scoring, routing decisions
* side effects: model calls, logging, network access, tool execution

That pattern makes systems much easier to test.

---

# 9. Closures: Functions as Values

A closure is a function-like value.

```rust
fn main() {
    let add_one = |x: i32| x + 1;

    let result = add_one(5);
    println!("{}", result);
}
```

## Unit test example

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn closure_can_be_called_like_a_function() {
        let add_one = |x: i32| x + 1;
        assert_eq!(add_one(5), 6);
    }
}
```

## Real use case

Closures are used everywhere in Rust:

* iterators
* callbacks
* retry logic
* filters
* agent pipeline steps

In practice, closures are how you inject small pieces of behavior into larger systems.

---

# 10. Passing Functions Into Functions

```rust
fn double(x: i32) -> i32 {
    x * 2
}

fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_works_with_plain_function() {
        assert_eq!(apply(double, 10), 20);
    }

    #[test]
    fn apply_works_with_closure() {
        let add_three = |x| x + 3;
        assert_eq!(apply(add_three, 7), 10);
    }
}
```

## Agentic use case

This is the beginning of pipeline design.

You can pass different processing behaviors into a common execution function.

For example:

* one function cleans text
* another validates JSON
* another scores relevance
* another routes to a tool

That is how systems become composable.

---

# 11. Returning Functions From Functions

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_adder_returns_working_closure() {
        let add_five = make_adder(5);
        assert_eq!(add_five(3), 8);
    }

    #[test]
    fn different_adders_have_different_state() {
        let add_two = make_adder(2);
        let add_ten = make_adder(10);

        assert_eq!(add_two(3), 5);
        assert_eq!(add_ten(3), 13);
    }
}
```

## Real use case

This is useful when building configurable behavior.

For example:

* build a scoring function with a threshold
* build a formatter with a prefix
* build a router with a policy

This is how you create behavior factories.

---

# 12. Generic Functions

```rust
fn identity<T>(value: T) -> T {
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_works_for_integer() {
        assert_eq!(identity(5), 5);
    }

    #[test]
    fn identity_works_for_str() {
        assert_eq!(identity("hello"), "hello");
    }
}
```

## Real use case

Generic functions let you reuse logic across many data types.

That is very useful in framework design and infrastructure code.

---

# 13. Trait Bounds

```rust
fn debug_string<T: std::fmt::Debug>(value: T) -> String {
    format!("{:?}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_string_formats_integer() {
        assert_eq!(debug_string(42), "42");
    }

    #[test]
    fn debug_string_formats_vector() {
        assert_eq!(debug_string(vec![1, 2]), "[1, 2]");
    }
}
```

## Real use case

Trait bounds let you define what a function requires.

In agent systems, you often want contracts like:

* must be serializable
* must be cloneable
* must be debug-printable
* must be sendable across threads

That is where trait bounds become powerful.

---

# 14. Iterator Pipelines

```rust
fn even_times_ten(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 10)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_times_ten_filters_and_maps() {
        assert_eq!(even_times_ten(vec![1, 2, 3, 4, 5, 6]), vec![20, 40, 60]);
    }

    #[test]
    fn even_times_ten_empty_input() {
        assert_eq!(even_times_ten(vec![]), Vec::<i32>::new());
    }
}
```

## Real use case

Iterator pipelines are excellent for:

* document preprocessing
* event filtering
* tool result transformation
* batch scoring
* structured extraction cleanup

This is one of Rust's most practical functional superpowers.

---

# 15. `Option` and `Result`

## `Option`

```rust
fn first_char(text: &str) -> Option<char> {
    text.chars().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_char_returns_some() {
        assert_eq!(first_char("rust"), Some('r'));
    }

    #[test]
    fn first_char_returns_none_for_empty_string() {
        assert_eq!(first_char(""), None);
    }
}
```

## `Result`

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("Could not parse `{s}`"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_returns_ok() {
        assert_eq!(parse_number("10"), Ok(10));
    }

    #[test]
    fn parse_number_returns_err() {
        assert!(parse_number("oops").is_err());
    }
}
```

## Agentic use case

This is essential in Rig-style agent architecture:

* `Option<T>` when data may be absent
* `Result<T, E>` when failure should be explicit and explainable

Example:

* no tool selected yet -> `Option<ToolChoice>`
* failed to parse extraction result -> `Result<StructuredOutput, ParseError>`

That is how typed workflows stay honest.

---

# 16. `map`, `and_then`, and `?`

## `map`

```rust
fn double_some(value: Option<i32>) -> Option<i32> {
    value.map(|x| x * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_some_transforms_some() {
        assert_eq!(double_some(Some(5)), Some(10));
    }

    #[test]
    fn double_some_preserves_none() {
        assert_eq!(double_some(None), None);
    }
}
```

## `and_then`

```rust
fn require_positive(n: i32) -> Result<i32, String> {
    if n > 0 {
        Ok(n)
    } else {
        Err("Number must be positive".to_string())
    }
}

fn parse_and_validate(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("Could not parse `{s}`"))
        .and_then(require_positive)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_validate_accepts_positive() {
        assert_eq!(parse_and_validate("42"), Ok(42));
    }

    #[test]
    fn parse_and_validate_rejects_negative() {
        assert!(parse_and_validate("-1").is_err());
    }

    #[test]
    fn parse_and_validate_rejects_invalid_text() {
        assert!(parse_and_validate("abc").is_err());
    }
}
```

## With `?`

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("Could not parse `{s}`"))
}

fn require_positive(n: i32) -> Result<i32, String> {
    if n > 0 {
        Ok(n)
    } else {
        Err("Number must be positive".to_string())
    }
}

fn half_of_positive(s: &str) -> Result<i32, String> {
    let n = parse_number(s)?;
    let n = require_positive(n)?;
    Ok(n / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_of_positive_success() {
        assert_eq!(half_of_positive("10"), Ok(5));
    }

    #[test]
    fn half_of_positive_fails_on_negative() {
        assert!(half_of_positive("-8").is_err());
    }

    #[test]
    fn half_of_positive_fails_on_invalid_input() {
        assert!(half_of_positive("abc").is_err());
    }
}
```

## Agentic use case

This is exactly how you write safe stages in an agent pipeline:

* parse input
* validate structure
* run domain rules
* continue only if each stage succeeds

Each stage is small.
Each failure is explicit.
That is how real systems survive contact with reality.

---

# 17. Memoization: Functions With Memory

```rust
use std::collections::HashMap;

fn memoized_square() -> impl FnMut(i32) -> i32 {
    let mut cache = HashMap::new();

    move |x| {
        if let Some(v) = cache.get(&x) {
            return *v;
        }

        let result = x * x;
        cache.insert(x, result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memoized_square_returns_correct_values() {
        let mut f = memoized_square();

        assert_eq!(f(4), 16);
        assert_eq!(f(4), 16);
        assert_eq!(f(5), 25);
    }

    #[test]
    fn each_memoized_closure_has_its_own_cache() {
        let mut f1 = memoized_square();
        let mut f2 = memoized_square();

        assert_eq!(f1(3), 9);
        assert_eq!(f2(3), 9);
    }
}
```

## Real use case

Memoization is useful for:

* repeated feature computation
* repeated prompt templating
* repeated embedding lookup
* repeated document normalization
* repeated deterministic tool planning

## Agentic use case in a Rig-style system

Imagine an agent repeatedly asks:

* Which schema applies to this document type?
* What is the normalized version of this vendor name?
* What tool policy is attached to this workflow?

If those computations are deterministic, memoization can save time and cost.

---

# 18. A Small Agentic Pipeline Example

Now let us build a tiny example inspired by agent pipelines in Rust.

This is not the full Rig framework API.
It is a teaching model that shows the same architectural thinking.

## Goal

We want a pipeline that:

* receives text
* validates it
* extracts a task
* returns typed output

```rust
#[derive(Debug, PartialEq, Eq)]
struct Task {
    title: String,
}

fn validate_input(text: &str) -> Result<&str, String> {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        Err("Input cannot be empty".to_string())
    } else {
        Ok(trimmed)
    }
}

fn extract_task(text: &str) -> Result<Task, String> {
    if text.to_lowercase().contains("email") {
        Ok(Task {
            title: "Send email".to_string(),
        })
    } else if text.to_lowercase().contains("schedule") {
        Ok(Task {
            title: "Schedule meeting".to_string(),
        })
    } else {
        Err("Could not infer task".to_string())
    }
}

fn run_pipeline(text: &str) -> Result<Task, String> {
    let valid = validate_input(text)?;
    let task = extract_task(valid)?;
    Ok(task)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_extracts_email_task() {
        let result = run_pipeline("Please email the client");
        assert_eq!(
            result,
            Ok(Task {
                title: "Send email".to_string()
            })
        );
    }

    #[test]
    fn pipeline_extracts_schedule_task() {
        let result = run_pipeline("Schedule a call with the team");
        assert_eq!(
            result,
            Ok(Task {
                title: "Schedule meeting".to_string()
            })
        );
    }

    #[test]
    fn pipeline_rejects_empty_input() {
        assert!(run_pipeline("   ").is_err());
    }

    #[test]
    fn pipeline_rejects_unknown_task() {
        assert!(run_pipeline("Think about philosophy").is_err());
    }
}
```

## Why this matters

This tiny example already shows the right shape:

* functions are small
* each function has one responsibility
* errors are explicit
* output is typed
* the pipeline is testable

That is the same design instinct you want in Rig-based systems.

---

# 19. Rig-Style Agent Function Design

When building agent systems in Rust, functions should usually sit around the model call like this:

## Stage 1: prepare prompt input

```rust
fn build_prompt(user_request: &str) -> String {
    format!("Extract the main task from this request:\n\n{user_request}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_prompt_contains_user_request() {
        let prompt = build_prompt("Email the finance team");
        assert!(prompt.contains("Email the finance team"));
    }
}
```

## Stage 2: parse structured output

```rust
#[derive(Debug, PartialEq, Eq)]
struct ExtractedTask {
    title: String,
}

fn parse_agent_output(text: &str) -> Result<ExtractedTask, String> {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return Err("Model returned empty output".to_string());
    }

    Ok(ExtractedTask {
        title: trimmed.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_agent_output_success() {
        let result = parse_agent_output("Send email");
        assert_eq!(
            result,
            Ok(ExtractedTask {
                title: "Send email".to_string()
            })
        );
    }

    #[test]
    fn parse_agent_output_empty_fails() {
        assert!(parse_agent_output("   ").is_err());
    }
}
```

## Stage 3: validate domain rules

```rust
fn validate_task(task: ExtractedTask) -> Result<ExtractedTask, String> {
    if task.title.len() < 3 {
        Err("Task title too short".to_string())
    } else {
        Ok(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_task_accepts_valid_title() {
        let task = ExtractedTask {
            title: "Send email".to_string(),
        };

        assert!(validate_task(task).is_ok());
    }

    #[test]
    fn validate_task_rejects_short_title() {
        let task = ExtractedTask {
            title: "Hi".to_string(),
        };

        assert!(validate_task(task).is_err());
    }
}
```

## Combined pipeline

```rust
#[derive(Debug, PartialEq, Eq)]
struct ExtractedTask {
    title: String,
}

fn build_prompt(user_request: &str) -> String {
    format!("Extract the main task from this request:\n\n{user_request}")
}

fn parse_agent_output(text: &str) -> Result<ExtractedTask, String> {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return Err("Model returned empty output".to_string());
    }

    Ok(ExtractedTask {
        title: trimmed.to_string(),
    })
}

fn validate_task(task: ExtractedTask) -> Result<ExtractedTask, String> {
    if task.title.len() < 3 {
        Err("Task title too short".to_string())
    } else {
        Ok(task)
    }
}

fn run_agent_stage(user_request: &str, simulated_model_output: &str) -> Result<ExtractedTask, String> {
    let _prompt = build_prompt(user_request);
    let parsed = parse_agent_output(simulated_model_output)?;
    let validated = validate_task(parsed)?;
    Ok(validated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_agent_stage_success() {
        let result = run_agent_stage(
            "Please help me follow up with the investor",
            "Send investor follow-up",
        );

        assert_eq!(
            result,
            Ok(ExtractedTask {
                title: "Send investor follow-up".to_string()
            })
        );
    }

    #[test]
    fn run_agent_stage_fails_on_empty_model_output() {
        assert!(run_agent_stage("Do something", "   ").is_err());
    }

    #[test]
    fn run_agent_stage_fails_on_invalid_title() {
        assert!(run_agent_stage("Do something", "Hi").is_err());
    }
}
```

## Why this is the right mindset for Rig

In a real Rig workflow, the LLM call sits in the middle.

But around it, you still want normal Rust functions for:

* input preparation
* typed parsing
* repair rules
* validation
* fallback selection
* telemetry-friendly boundaries

The model should be one stage, not the whole architecture.

---

# 20. A More Realistic Agentic Example: Tool Routing

Here is a tiny routing example.

```rust
#[derive(Debug, PartialEq, Eq)]
enum ToolChoice {
    Email,
    Calendar,
    Search,
}

fn choose_tool(intent: &str) -> Result<ToolChoice, String> {
    let intent = intent.to_lowercase();

    if intent.contains("email") {
        Ok(ToolChoice::Email)
    } else if intent.contains("meeting") || intent.contains("schedule") {
        Ok(ToolChoice::Calendar)
    } else if intent.contains("find") || intent.contains("search") {
        Ok(ToolChoice::Search)
    } else {
        Err("No matching tool".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choose_tool_email() {
        assert_eq!(choose_tool("send email to client"), Ok(ToolChoice::Email));
    }

    #[test]
    fn choose_tool_calendar() {
        assert_eq!(choose_tool("schedule a meeting"), Ok(ToolChoice::Calendar));
    }

    #[test]
    fn choose_tool_search() {
        assert_eq!(choose_tool("search for rust docs"), Ok(ToolChoice::Search));
    }

    #[test]
    fn choose_tool_unknown() {
        assert!(choose_tool("ponder the universe").is_err());
    }
}
```

## Real use case

This is exactly the kind of function that sits beside an agent framework:

* take an interpreted intent
* map it to a typed tool choice
* fail clearly when no valid route exists

Very small function.
Very big architectural value.

---

# 21. Practice Exercises

## Exercise 1: basic return

```rust
fn double(n: i32) -> i32 {
    // return n * 2
}
```

## Exercise 2: add tests to a pure function

Write tests for this:

```rust
fn triple(n: i32) -> i32 {
    n * 3
}
```

## Exercise 3: parse and validate user input

Write:

```rust
fn parse_age(s: &str) -> Result<u8, String>
```

Rules:

* parse number
* age must be between 0 and 120

## Exercise 4: tiny agent router

Write:

```rust
fn choose_tool(intent: &str) -> Result<&'static str, String>
```

Map:

* "email" -> "email_tool"
* "schedule" -> "calendar_tool"
* "search" -> "search_tool"

## Exercise 5: memoized computation

Write:

```rust
fn memoize_double() -> impl FnMut(i32) -> i32
```

And add tests.

---

# 22. Final Cheat Sheet

## Function basics

```rust
fn name(inputs) -> output { body }
```

## Final expression rule

* last expression without `;` returns
* `;` discards the value
* `return expr;` is explicit return

## Ownership in parameters

* `&T` = borrow
* `&mut T` = mutable borrow
* `T` = take ownership

## Methods

* `&self` = read
* `&mut self` = modify
* `self` = consume

## Closures

* closure = function + captured environment
* `move` = capture by ownership

## Closure traits

* `Fn` = read captured state
* `FnMut` = mutate captured state
* `FnOnce` = consume captured state

## Iterator trio

* `iter()` = borrow items
* `iter_mut()` = mutably borrow items
* `into_iter()` = consume items

## Functional combinators

* `map` = transform
* `filter` = keep matching
* `filter_map` = transform valid values, drop the rest
* `flat_map` = one to many
* `fold` = reduce many to one
* `and_then` = chain fallible or wrapped computation

## Errors

* `Option<T>` = maybe a value
* `Result<T, E>` = success or error
* `?` = propagate error early

## Lifetimes

* describe borrow relationships
* do not extend lifetimes

## Memoization

* cache stores input -> output
* closure owns cache
* cache lives as long as the closure lives
* use `FnMut` if the closure mutates the cache
* use `Arc<Mutex<_>>` for shared thread-safe cache

## Agentic Rust design rule

* keep model calls as one stage
* keep parsing, validation, routing, and fallback as normal Rust functions
* test those functions heavily
* treat function signatures as system contracts

---

# Final Takeaway

The beginner view is:

> a function is reusable code

The Rust view is stronger:

> a function is a typed contract about computation, ownership, and behavior

And the systems view is stronger still:

> a function is a boundary for correctness, testing, composition, and reliability

That is especially true in agent systems.

In a serious Rust agent architecture, functions are not just helpers.

They are the things that stop your pipeline from turning into expensive chaos.

If you want runnable code now, continue with:

- [Examples and Scripts](./examples-and-scripts.md)
- [Tutorial Path](./tutorial.md)
- [Quality Gates](./quality-gates.md)

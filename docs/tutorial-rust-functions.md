# Tutorial: Build Reliable Rust Function Thinking (Step by Step)

This tutorial follows a first-principles progression: functions as typed contracts, then ownership-aware behavior, and finally stateful, production-aware units.

## Goal

- Learn to design, test, and compose Rust functions from first principles.
- Understand when to use borrows, ownership, closures, iterators, and memoization.
- Build a full parse -> validate -> transform -> collect pipeline with predictable behavior.

## Prerequisites

- Rust installed and `cargo` working.
- You can read and run Rust code from this repository.
- You have completed `cargo test --all-targets` at least once on this repo.

## Steps

1. Open the project entry points so you understand the module map.
   - [`src/lib.rs`](../src/lib.rs)
   - [`src/lessons/mod.rs`](../src/lessons/mod.rs)

2. Start at the first principle: a function is a contract.
   - Read:
     - [`src/lessons/lesson_core.rs`](../src/lessons/lesson_core.rs)
   - Observe:
     - input/output in signature
     - no side effects in small helpers when not required
   - Run:

   ```bash
   cargo run --example lesson_basic
   ```

3. Learn what ownership in the signature changes.
   - Read:
     - [`src/lessons/ownership.rs`](../src/lessons/ownership.rs)
   - Focus on:
     - borrowed `&str` vs owned `String`
     - `&self`, `&mut self`, `self`
   - Run:

   ```bash
   cargo run --example lesson_ownership
   ```

4. Treat behavior like data: closures and function passing.
   - Read:
     - [`src/lessons/closures.rs`](../src/lessons/closures.rs)
   - Compare `fn` pointer and closure call patterns.
   - Run:

   ```bash
   cargo run --example lesson_closures
   ```

5. Read transformations as pipelines.
   - Read:
     - [`src/lessons/iterators.rs`](../src/lessons/iterators.rs)
   - Run:

   ```bash
   cargo run --example lesson_iterators
   ```

6. Introduce fallibility deliberately.
   - Read:
     - [`src/lessons/error_handling.rs`](../src/lessons/error_handling.rs)
   - Observe `Result`, `map`, `and_then`, and `?`.
   - Run:

   ```bash
   cargo run --example lesson_error_handling
   ```

7. Build and explain memoization in two levels.
   - Read:
     - [`src/lessons/memoization.rs`](../src/lessons/memoization.rs)
   - Run:

   ```bash
   cargo run --example lesson_memoization
   ```

8. Compose all previous layers into one pipeline.
   - Read:
     - [`src/lessons/pipeline.rs`](../src/lessons/pipeline.rs)
   - Run:

   ```bash
   cargo run --example lesson_pipeline
   ```

9. Run the full module-level checks.

   ```bash
   cargo test --all-targets
   cargo check --all-targets
   ```

10. Validate quality gates exactly as CI does.

   ```bash
   scripts/quality.sh
   ```

11. Extend one example without looking at the solution.

   - Add an extra validation rule in [`src/lessons/error_handling.rs`](../src/lessons/error_handling.rs), for example:
     - reject inputs above a configurable maximum.
   - Update:
     - unit tests in the same file.
     - or add a new example in [`examples/`](../examples/).

12. Re-run targeted checks.

   ```bash
   cargo test
   cargo run --example lesson_error_handling
   ```

## Expected behavior and checks

- `cargo run --example lesson_basic` prints three deterministic lines from the most basic APIs.
- `cargo run --example lesson_memoization` prints the same number twice and should show cache reuse behavior.
- `cargo run --example lesson_pipeline` returns:
  - success for valid inputs: `[40, 70, 40]` for `vec![\"4\", \"7\", \"4\"]`
  - error path on invalid or negative input.

## Notes for instructors

- Each lesson function is intentionally small.
- If a student fails a step, run the function-level unit tests in that same file first.
- Emphasize one rule repeatedly: **make failure visible in types (`Option`/`Result`) before making control flow complex**.

## Related scripts

- [`scripts/quality.sh`](../scripts/quality.sh): full local quality gate.

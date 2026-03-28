# Rust Education Playbook for This Repository

## Audience

- Primary audience: learners with basic programming fluency in any language.
- Minimum Rust context: syntax for variables, functions, and `Vec`.
- Typical session formats: self-paced 30–45 minutes per phase, or 2-hour cohort workshop.

## Objectives

By completing this repo:

1. Read function signatures as contracts (inputs, outputs, ownership, and fallibility).
2. Compare borrows and owned values in real call sites.
3. Compose behavior with closures, traits, and iterators.
4. Use `Result` and `Option` combinators with predictable failure behavior.
5. Implement memoization with explicit cache ownership and predictable state lifetime.
6. Build end-to-end pipelines that are test-driven and CI-safe.

## Artifacts

- Source lesson modules:
  - [`src/lessons/lesson_core.rs`](../src/lessons/lesson_core.rs)
  - [`src/lessons/ownership.rs`](../src/lessons/ownership.rs)
  - [`src/lessons/closures.rs`](../src/lessons/closures.rs)
  - [`src/lessons/iterators.rs`](../src/lessons/iterators.rs)
  - [`src/lessons/error_handling.rs`](../src/lessons/error_handling.rs)
  - [`src/lessons/memoization.rs`](../src/lessons/memoization.rs)
  - [`src/lessons/pipeline.rs`](../src/lessons/pipeline.rs)
- Executable examples:
  - [`examples/lesson_basic.rs`](../examples/lesson_basic.rs)
  - [`examples/lesson_ownership.rs`](../examples/lesson_ownership.rs)
  - [`examples/lesson_closures.rs`](../examples/lesson_closures.rs)
  - [`examples/lesson_iterators.rs`](../examples/lesson_iterators.rs)
  - [`examples/lesson_error_handling.rs`](../examples/lesson_error_handling.rs)
  - [`examples/lesson_memoization.rs`](../examples/lesson_memoization.rs)
  - [`examples/lesson_pipeline.rs`](../examples/lesson_pipeline.rs)
- Quality gates:
  - [`scripts/quality.sh`](../scripts/quality.sh)
  - `README.md` and [`LESSONS.md`](../LESSONS.md) for orientation.

## Assessment

1. Unit tests per module (existing `#[cfg(test)]` blocks) must pass.
2. New examples must compile and run with:

   ```bash
   cargo run --example lesson_pipeline
   ```

3. Added lesson modifications are accepted only after:
   - `cargo test --all-targets --all-features`
   - `cargo test`
   - `scripts/quality.sh` (local equivalent of CI gates)
4. Concept checks:
   - Can learner explain why `borrow_name` keeps ownership in caller?
   - Can learner trace `pipeline::process` failure propagation with one invalid token?
   - Can learner add an exercise with one `TODO` behavior and a matching assertion?

## Risks

- Risk: learners confuse `Fn`, `FnMut`, and `FnOnce` in closures.
  - Mitigation: always show one closure example with explicit state update before moving on.
- Risk: ownership errors from passing owned `String` where a borrow is required.
  - Mitigation: use side-by-side `borrow_name` vs `take_name` and keep both in exercises.
- Risk: iterator laziness confusion when no terminal method is present.
  - Mitigation: instrument with `println!` inside `map` then demonstrate `collect`.
- Risk: cache behavior confusion with memoization.
  - Mitigation: compare first-call/second-call outputs and call out closure-owned state.

## Next Step

1. Pick one real function from your own codebase and rewrite it in the same ladder:
   - pure function signature,
   - test,
   - closure-based composition,
   - error type redesign.
2. Add it as a new lesson module and add an example that prints expected output.
3. Run:

   ```bash
   cargo test
   scripts/quality.sh
   ```

## Learning progression used in this repo

Phase 1 — Foundations
- `lesson_core`, `lesson_ownership`

Phase 2 — Mechanics
- `closures`, `iterators`, `error_handling`

Phase 3 — Architecture and integration
- `memoization`, `pipeline`

Each phase ends with at least one runnable example and one unit-test verification cycle.

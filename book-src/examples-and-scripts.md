# Examples and Scripts

Run every example to make the lesson concrete.

## Lesson examples

```bash
cargo run --example lesson_basic
cargo run --example lesson_ownership
cargo run --example lesson_closures
cargo run --example lesson_iterators
cargo run --example lesson_error_handling
cargo run --example lesson_memoization
cargo run --example lesson_pipeline
```

## Useful scripts

- `scripts/quality.sh` — local quality gate for format, lint, tests, and examples.
- `scripts/build-mdbook.sh` — builds the documentation output.

## Code and tests mapping

- [`src/lessons/lesson_core.rs`](../src/lessons/lesson_core.rs) with tests for:
  - function signatures and composition
- [`src/lessons/ownership.rs`](../src/lessons/ownership.rs) with tests for:
  - borrow semantics and ownership transitions
- [`src/lessons/memoization.rs`](../src/lessons/memoization.rs) with tests for:
  - cache hit, cache miss, and deterministic behavior

## How to add a new runnable example

1. Add `examples/your_lesson.rs`.
2. Include clear `println!` checkpoints and comments that explain intent.
3. Ensure it compiles by running:

```bash
cargo build --example your_lesson
```

4. Add a matching mention in:
   - [`README.md`](../README.md)
   - [`examples-and-scripts`](./examples-and-scripts.md)
   - [`lesson-map`](./lesson-map.md)

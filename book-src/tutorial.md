# Tutorial Path (From Beginner to Advanced)

This course is written as an incremental progression.

## 1) Function contracts first

Read and run:

- [`src/lessons/lesson_core.rs`](../src/lessons/lesson_core.rs)
- [`examples/lesson_basic.rs`](../examples/lesson_basic.rs)

Focus question: **What is the type-level promise at the function boundary?**

## 2) Ownership and method receivers

Read and run:

- [`src/lessons/ownership.rs`](../src/lessons/ownership.rs)
- [`examples/lesson_ownership.rs`](../examples/lesson_ownership.rs)

Focus question: **Does this function borrow, mutably borrow, or take ownership?**

## 3) Closures and behavior as values

Read and run:

- [`src/lessons/closures.rs`](../src/lessons/closures.rs)
- [`examples/lesson_closures.rs`](../examples/lesson_closures.rs)

Focus question: **Does this callable borrow state, mutate it, or consume it?**

## 4) Iterator pipelines

Read and run:

- [`src/lessons/iterators.rs`](../src/lessons/iterators.rs)
- [`examples/lesson_iterators.rs`](../examples/lesson_iterators.rs)

Focus question: **Can I describe each stage as data transformation, not manual loops?**

## 5) Fallible composition

Read and run:

- [`src/lessons/error_handling.rs`](../src/lessons/error_handling.rs)
- [`examples/lesson_error_handling.rs`](../examples/lesson_error_handling.rs)

Focus question: **Can I push failure handling into clear typed boundaries (`Result`/`Option`)?**

## 6) Memoization and stateful compute units

Read and run:

- [`src/lessons/memoization.rs`](../src/lessons/memoization.rs)
- [`examples/lesson_memoization.rs`](../examples/lesson_memoization.rs)

Focus question: **Who owns the cache, and what is its lifetime?**

## 7) End-to-end pipeline

Read and run:

- [`src/lessons/pipeline.rs`](../src/lessons/pipeline.rs)
- [`examples/lesson_pipeline.rs`](../examples/lesson_pipeline.rs)

Focus question: **Can this pipeline be tested with valid, invalid, and boundary inputs?**

## Recommended study sequence

Do not jump forward early. A failed check in one stage usually means the previous model is not stable yet.

```bash
# Example: run one module + full validation
cargo test --all-targets
cargo run --example lesson_pipeline
```

If you want a stricter local gate:

```bash
scripts/quality.sh
```

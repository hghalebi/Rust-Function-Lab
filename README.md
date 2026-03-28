# Rust Functions — Practical Learning Path (Inspired by Andrew Ng’s style)

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)

[![Build Status](https://img.shields.io/badge/CI-Active-brightgreen.svg)](https://github.com/hghalebi/Rust-Function-Lab/actions)

Welcome to a hands-on Rust course where every concept is connected to a concrete problem.

This repository is intentionally built around one principle:

**learn one powerful mental model at a time, then compose them into larger systems**.

You won’t just read syntax. You will build small, test-verified pieces and reuse them to form bigger programs.

---

## What you will learn

By the end, you should be able to:

1. Write clear functions with correct input/output contracts.
2. Read Rust ownership and borrowing in function signatures.
3. Use methods and trait-based behavior to model domain logic.
4. Compose behavior using closures, function pointers, and iterator pipelines.
5. Handle fallibility with `Result` and build resilient pipelines.
6. Build memoized functions that remember previous computations.
7. Protect correctness with tests and continuous quality checks.

This repo turns your provided curriculum into **executable lessons** and **verifiable examples**.

---

## How this repository is organized

### Core library

- [`src/lib.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/src/lib.rs)
  - Public library entry point for lesson modules.
- [`src/lessons/mod.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/src/lessons/mod.rs)
  - Module map for all lesson groups.

### Lesson modules

- [`src/lessons/lesson_core.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/src/lessons/lesson_core.rs)
  - Basic function signatures, composition, composition helper, tuple-struct destructuring style.
- [`src/lessons/ownership.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/src/lessons/ownership.rs)
  - Ownership, borrowing, `&self` vs `&mut self` vs `self`, plus pattern matching basics.
- [`src/lessons/closures.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/src/lessons/closures.rs)
  - Closures, closure trait behavior (`Fn`, `FnMut`, `FnOnce`), and higher-order function examples.
- [`src/lessons/iterators.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/src/lessons/iterators.rs)
  - Iterator pipelines (`map`, `filter`, `filter_map`, `flat_map`, `sum`).
- [`src/lessons/error_handling.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/src/lessons/error_handling.rs)
  - `Result`, `Option`, `?`, and safe fallible composition.
- [`src/lessons/memoization.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/src/lessons/memoization.rs)
  - Memoization from scratch, generic memoizer, thread-safe cache variant.
- [`src/lessons/pipeline.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/src/lessons/pipeline.rs)
  - End-to-end parser/validator/transform pipeline with memoized compute.

### Runnables

- [`examples/lesson_basic.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/examples/lesson_basic.rs)
  - Core entry-level functions.
- [`examples/lesson_ownership.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/examples/lesson_ownership.rs)
  - Borrowing and method receiver behavior.
- [`examples/lesson_closures.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/examples/lesson_closures.rs)
  - Higher-order and closure-based programming.
- [`examples/lesson_iterators.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/examples/lesson_iterators.rs)
  - Functional sequence transformations.
- [`examples/lesson_error_handling.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/examples/lesson_error_handling.rs)
  - Parsing + validation examples with `Result`.
- [`examples/lesson_memoization.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/examples/lesson_memoization.rs)
  - Stateful closures and cache reuse behavior.
- [`examples/lesson_pipeline.rs`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/examples/lesson_pipeline.rs)
  - Full production-style pipeline example.

### CI and quality automation

- [`/.github/workflows/ci.yml`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/.github/workflows/ci.yml)
  - Runs formatting check, clippy, tests, compile checks, and example builds on push/PR.
- [`scripts/quality.sh`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/scripts/quality.sh)
  - One command to validate the same gates used in CI.

---

## Recommended learning sequence

This is the path to follow to get the strongest outcome:

### Phase 1 — Foundations

1. Open `lesson_basic`.
2. Trace `say_hello`, `greet`, and `add`.
3. Write one line of your own and run tests.

```bash
cargo test --all-targets
cargo run --example lesson_basic
```

### Phase 2 — Ownership-aware thinking

1. Open `ownership` lesson and examples.
2. Compare `borrow_name` and `take_name` to see move vs borrow in practice.
3. Extend `Counter` with a new method and ensure tests still pass.

### Phase 3 — Behaviors as values

1. Open `closures` lesson and `lesson_closures`.
2. Use closures with state and observe when `Fn`, `FnMut`, and `FnOnce` are required.

### Phase 4 — Functional data flow

1. Open `iterators` lesson and `lesson_iterators`.
2. Build a pipeline with `filter -> map -> flat_map -> sum`.
3. Add tests for every intermediate stage.

### Phase 5 — Reliable, fail-safe programs

1. Open `error_handling`.
2. Replace nested matches with `?`.
3. Add a new invalid input test and make it pass.

### Phase 6 — Stateful computation units

1. Open `memoization` and `lesson_memoization`.
2. Reproduce memoized square, then convert it to generic `memoize`.
3. Compare cache behavior between plain and thread-safe versions.

### Phase 7 — Integration

1. Open `pipeline` and `lesson_pipeline`.
2. Build a full parse -> validate -> transform -> collect flow.
3. Add a negative/invalid case test.

---

## How to run everything

```bash
# Run unit tests for all lesson modules
cargo test --all-targets

# Compile + run static checks + example builds
scripts/quality.sh

# Run a single lesson
cargo run --example lesson_pipeline
```

## Build the mdBook documentation

- Install:

```bash
cargo install mdbook
```

- Build and serve locally:

```bash
mdbook build
mdbook serve --open
```

- Or use the helper:

```bash
scripts/build-mdbook.sh
```

- The generated book output appears in `book/`.

For incremental checks while learning:

```bash
cargo test
cargo run --example lesson_iterators
```

---

## Curriculum quality and correctness model

Every module is built with:

- small pure-ish functions where possible,
- explicit tests for each behavior,
- examples that exercise public usage,
- CI that fails the build if any lesson regresses.

Additions should follow the same pattern:

1. Add/extend lesson module under `src/lessons/`.
2. Add or update unit tests inside the module.
3. Add an example in `examples/`.
4. Run `scripts/quality.sh` locally.

## Pedagogical documentation

- [Tutorial: Build Reliable Rust Function Thinking (Step by Step)](docs/tutorial-rust-functions.md)
- [Rust-edu learning playbook](docs/rust-edu-learning-playbook.md)
- [Quality gate script](scripts/quality.sh)

---

## Why this course is structured this way

Following the sequence above mirrors how strong intuition forms:

- from single-purpose functions,
- to contracts and ownership,
- to composability,
- to resilient pipelines,
- to stateful, production-aware abstractions.

That is the same progression used in serious ML/MLSystems courses: build a small, correct abstraction, prove it with tests, then compose it into bigger systems.

---

## Extending the lesson set

If you add a new section:

- Put reusable core logic in `src/lessons/`.
- Add targeted unit tests in that same module.
- Add a matching example in `examples/`.
- Update module exports if needed.
- Re-run `scripts/quality.sh` and commit only when green.

---

## Notes

- If you run examples in a fresh clone, Rust will resolve everything from this repo only.
- CI uses GitHub Actions in [`/.github/workflows/ci.yml`](/Users/hamzeghalebi/projects/learning/Rust/RustFword/.github/workflows/ci.yml) on each push and pull request.

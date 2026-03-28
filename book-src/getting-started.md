# Getting Started

This chapter is your launchpad.  
The goal is simple: move from syntax recall to a reusable **engineering pattern** for function design.

## Install and prerequisites

```bash
cargo --version
cargo install mdbook
```

## Repository learning entry points

- `src/lib.rs` — package entry for lesson modules.
- `src/lessons/mod.rs` — lesson module registry.
- `src/lessons/*.rs` — one lesson cluster per chapter.
- `examples/*.rs` — runnable illustrations for each lesson cluster.
- `.github/workflows/ci.yml` — CI quality checkpoints.
- `scripts/quality.sh` — local quality command.

## Your first run

Run these in order:

```bash
cargo test --all-targets
cargo run --example lesson_basic
cargo run --example lesson_memoization
```

If these work, the project is in a good baseline state.

## First principles checklist

1. What data does a function take?
2. What does it promise to return?
3. Who owns inputs and outputs?
4. Can it fail? If yes, which error path is exposed?
5. Is the behavior composable?
6. Can this be tested as a small, isolated unit?

## Suggested cadence (30 minutes)

- 10 minutes: read one lesson in `src/lessons`.
- 10 minutes: mirror one example in `examples`.
- 10 minutes: add one test variant and rerun targeted checks.

This rhythm reinforces retention better than reading long pages in one pass.

# Contributing

Thanks for your interest in improving this repository.

This project is a learning-focused Rust curriculum with executable lessons,
examples, and mdBook documentation. Contributions that improve clarity, correctness,
and test quality are especially welcome.

## Development setup

1. Install Rust.
2. Run:

   ```bash
   cargo test --all-targets --all-features
   scripts/quality.sh
   ```

3. Build docs if you change tutorial content:

   ```bash
   mdbook build
   ```

## Code style

- Prefer readable, minimal examples.
- Keep teaching examples small and test-driven.
- Keep side effects at edges; prefer pure logic in lesson functions.
- Use unit tests inside lesson modules when adding or changing behavior.
- Avoid TODOs in public examples; either complete behavior or skip the change.

## Commit message

- Use concise, imperative commit messages.
- Mention behavior changes and test impact in the body when needed.

## Before opening a PR

- Ensure `cargo fmt`, `cargo clippy --all-targets --all-features`, and
  `cargo test --all-targets --all-features` pass.
- Run `mdbook build` if documentation changed.

## Reporting issues

- Open an issue with:
  - the affected lesson/example
  - reproduction steps
  - expected behavior
  - test output if applicable

# Quality Gates and Correctness

This project treats correctness as a prerequisite for educational completeness.

## Minimum quality bar

- All lesson modules compile on Rust stable.
- Unit tests validate expected behavior and edge cases.
- Examples execute and match lesson expectations.
- CI checks pass (`format`, `clippy`, `test`).

## Local command sequence

```bash
cargo fmt
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
scripts/quality.sh
```

## When a check fails

Use this order:

1. Fix compilation errors in lesson modules first.
2. Update failing tests only if behavior was intentionally changed.
3. Re-run only the impacted example.
4. Re-run full checks before moving to the next lesson.

## mdBook generation

```bash
mdbook build
mdbook serve --no-open --port 4000
```

Generated output is written to `book/` and served from that directory by mdBook.

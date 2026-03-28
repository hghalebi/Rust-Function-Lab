# Rust Functions — Pedagogical Curriculum

This course follows your provided content structure from first principles to production-ready patterns.

## Quick documentation entry points

- [Step-by-step tutorial](docs/tutorial-rust-functions.md)
- [Rust-edu learning playbook](docs/rust-edu-learning-playbook.md)

## Learning arc

1. `say_hello`, `greet`, and `add` (core function signatures)
2. Ownership in function parameters (`&str` vs `String`)
3. Methods and `self` variants (`&self`, `&mut self`, `self`)
4. Closures and closure traits (`Fn`, `FnMut`)
5. Iterators and pipeline transformations
6. `Option` and `Result` composition (`map`, `and_then`, `?`)
7. Generic functions, trait bounds, and trait-based behavior
8. Memoization with closures and internal cache ownership
9. End-to-end pipeline that combines parsing, validation, and compute

## Module mapping

All code is intentionally minimal and tested. If you add a lesson:

- add a new function module under `src/lessons/`
- add tests in that module
- add an example under `examples/` to ensure compile-time verification of usage
- keep `src/lib.rs` exports stable and explicit

For details, run `cargo test` after each lesson update.

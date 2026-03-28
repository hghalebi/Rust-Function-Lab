# Rust Functions: A Hands-On Learning Path

Welcome. This book is a **curriculum-first** Rust course that teaches functions from the
foundations to production-aware patterns.

You are going to build one shared mental model:

1. A function is an executable contract.
2. Ownership is part of the contract.
3. Pure transformations compose best.
4. Stateful behavior can be wrapped safely with explicit ownership.
5. Correctness is non-optional (tests + CI).

This is designed to be **readable, testable, and directly connect to code**.

## What this book contains

- A staged curriculum from `say_hello()` to memoized closures.
- Direct connections to executable code in:
  - `src/lessons/*` reusable lesson modules
  - `examples/*` runnable scripts
- A quality gate model that protects examples from drift.

## Start here

1. Read [Getting Started](./getting-started.md) to orient yourself.
2. Open [Tutorial Path](./tutorial.md) and read the roadmap with your course files.
3. Use [Examples and Scripts](./examples-and-scripts.md) to run code immediately.
4. Run [Quality Gates](./quality-gates.md) regularly.

## How to study this book

- Do not read only. Type, run, break, and fix each lesson.
- Use the “why” comments in code blocks and examples to understand design intent.
- After every lesson, run the corresponding tests or examples.

## Repository anchors

- [Course overview](../README.md)
- [Tutorial notes](../docs/tutorial-rust-functions.md)
- [Learning playbook](../docs/rust-edu-learning-playbook.md)
- [CI and quality checks](../scripts/quality.sh)

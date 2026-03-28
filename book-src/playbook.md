# Pedagogical Playbook

This is the instructor-style guide for how the curriculum is structured.

## Learning design principle

Each lesson follows this sequence:

1. **Intuition statement** — one paragraph with the idea in plain language.
2. **Minimal syntax form** — 3–10 lines of code.
3. **Reasoning checks** — ownership, error model, or composition constraints.
4. **Mini exercise** — small change and immediate execution.
5. **Unit test update** — add one new edge case.
6. **Reflection note** — one sentence about why this works.

That sequence is repeated across all lesson modules.

## How to keep quality high

- Keep examples short enough to compile mentally.
- Keep tests focused on behavior, not implementation details.
- Keep names semantically honest (`parse` should parse, `validate` should only validate).
- Prefer immutable values where possible.
- Capture state only when behavior truly needs it.

## Suggested micro-rituals (Andrew Ng-style pacing)

- 1 idea, 1 exercise, 1 test.
- 3 minutes of prediction: “What should this output?”
- 5 minutes of implementation.
- 2 minutes of edge-case expansion.
- 1 minute of cleanup.

## Anti-patterns to avoid

- Teaching ownership without explicit ownership diagrams.
- Returning plain `String` errors instead of typed error shapes.
- Describing semantics without running the code.
- Adding features without adding tests.

## Instructor note

When a learner asks “why this is hard,” answer with the contract first.
The code usually becomes obvious after the contract is fixed.

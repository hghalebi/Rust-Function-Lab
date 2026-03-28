# Lesson Map

This map links curriculum topics to concrete files and validation tasks.

## Module map

| Topic | Source | Example | Primary learning objective |
|---|---|---|---|
| Function foundations | `src/lessons/lesson_core.rs` | `examples/lesson_basic.rs` | Signatures, returns, composition |
| Ownership & borrowing | `src/lessons/ownership.rs` | `examples/lesson_ownership.rs` | Borrow vs move semantics |
| Closures | `src/lessons/closures.rs` | `examples/lesson_closures.rs` | `Fn/FnMut/FnOnce` behavior |
| Iterators | `src/lessons/iterators.rs` | `examples/lesson_iterators.rs` | Declarative data pipelines |
| Error handling | `src/lessons/error_handling.rs` | `examples/lesson_error_handling.rs` | `Result` and fallible composition |
| Memoization | `src/lessons/memoization.rs` | `examples/lesson_memoization.rs` | Stateful closures, cache lifetime |
| Full pipeline | `src/lessons/pipeline.rs` | `examples/lesson_pipeline.rs` | Parse + validate + transform composition |

## Additions checklist

When adding a new lesson:

1. Add module in `src/lessons/`.
2. Add unit tests in same module.
3. Add one runnable example.
4. Add links in:
   - this map
   - `README.md`
   - `SUMMARY.md`
5. Run:

```bash
scripts/quality.sh
```

## Learning flow reminder

Each later lesson should reuse at least one earlier idea.  
This creates transfer, not memorization.

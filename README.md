# Learning Rust

A personal learning journal for Rust — built incrementally, one concept at a time.

Each topic starts with a question or a compiler error. The notes capture not just
the answer but the *why* behind it — the mental models that make Rust click.

---

## Approach

Rather than working through a textbook linearly, this repo grows from real
exploration: writing code, hitting errors, understanding what the compiler is
telling us, and building up intuition from first principles.

Notes are written to be revisited. The goal is internalization, not just syntax.

---

## Topics

### Ownership
The foundation of Rust. Most of the early lessons live here because ownership,
borrowing, and memory management are what make Rust different from everything else.

| File | Topic |
|------|-------|
| `ownership/01_stack_and_heap.md` | How memory is laid out; what lives where and why |
| `ownership/02_ownership_rules.md` | The three rules that govern every value in Rust |
| `ownership/03_references_and_borrowing.md` | Borrowing without taking ownership; `&T` and `&mut T` |
| `ownership/04_iterators_and_mut.md` | Why iterators must be `mut` even when the data isn't |
| `ownership/05_strings.md` | `&str` vs `&String`, fat vs thin pointers, deref coercion |
| `ownership/06_iterators_lazy_eval.md` | Lazy evaluation, adaptors vs consumers, infinite iterators |
| `ownership/07_iter_vs_into_iter.md` | `.iter()` vs `.into_iter()`, two-step vs single pipeline |

---

## Running the Examples

Examples are in `ownership/src/`. Compile any file directly with `rustc`:

```
rustc ownership/src/borrow_demo.rs && ./borrow_demo
```

---

## Notes Format

Each file follows the same pattern:

- A concrete example first
- The mental model that explains it
- The rule generalized from the model
- Comparisons to other languages where helpful (Elixir, etc.)

# Iterators: Lazy Evaluation and Consumers

## Lazy vs Eager

Iterators in Rust are **lazy** — calling `.map()`, `.filter()`, `.take()` etc.
builds a description of work but does nothing. Work only happens when a
**consumer** drives the pipeline.

If you've used Elixir, the mental model maps directly:

| Elixir | Rust | Behavior |
|--------|------|----------|
| `Enum` | — | eager: does work immediately |
| `Stream` | `Iterator` | lazy: describes work, waits for a consumer |
| `Enum.to_list/1` | `.collect()` | consumer: triggers the work |

---

## Building a Pipeline

```rust
let v = vec![1, 2, 3, 4, 5];

// Nothing happens here — just building a pipeline
let pipeline = v.iter()
    .map(|x| x * 2)
    .filter(|x| x > &4);

// Work happens HERE — the consumer drives it
let result: Vec<i32> = pipeline.collect();
// result: [6, 8, 10]
```

Equivalent in Elixir:

```elixir
[1, 2, 3, 4, 5]
|> Stream.map(&(&1 * 2))       # no work yet
|> Stream.filter(&(&1 > 4))    # no work yet
|> Enum.to_list()              # work happens here
```

---

## Why Lazy? Efficiency.

Eager evaluation allocates intermediate collections at every step.
Lazy evaluation flows elements through the pipeline one at a time.

```
Eager:
  [1,2,3,4,5] → map → [2,4,6,8,10] → filter → [6,8,10]
                        ^^^^^^^^^^^
                        entire Vec allocated just to be discarded

Lazy:
  1 → map → 2  → filter → drop
  2 → map → 4  → filter → drop
  3 → map → 6  → filter → keep
  4 → map → 8  → filter → keep
  5 → map → 10 → filter → keep
  result: [6, 8, 10]  ← only this is allocated
```

No intermediate allocations. Elements are processed one at a time and either
passed forward or discarded.

---

## Why Lazy? Infinite Iterators.

Laziness makes infinite sequences possible — you only pull what you ask for.

```rust
let first_five_evens: Vec<i32> = (0..)        // infinite range — no upper bound
    .filter(|x| x % 2 == 0)
    .take(5)                                  // stop after 5 matches
    .collect();
// [0, 2, 4, 6, 8]
```

An eager approach would try to evaluate the infinite range before filtering —
impossible. Laziness lets `.take(5)` act as a brake: once 5 elements pass
through, the whole pipeline stops.

---

## Consumers

A consumer is what triggers evaluation. Without one, the pipeline is inert.

| Consumer | What it produces |
|----------|-----------------|
| `.collect()` | pulls all elements into a collection (`Vec`, `HashMap`, etc.) |
| `.sum()` | adds all elements |
| `.product()` | multiplies all elements |
| `.count()` | counts elements |
| `.for_each(f)` | runs a closure on each element, returns nothing |
| `.find(f)` | returns the first element matching a predicate |
| `.any(f)` | returns `true` if any element matches |
| `.all(f)` | returns `true` if all elements match |
| `.next()` | pulls exactly one element (returns `Option`) |
| `.last()` | pulls all elements, returns the last |
| `.max()` / `.min()` | pulls all elements, returns the extreme |

`.next()` is the primitive — every other consumer is built on top of it.

---

## Adaptors vs Consumers

**Adaptors** transform the pipeline and return another iterator (lazy):
`.map()`, `.filter()`, `.take()`, `.skip()`, `.zip()`, `.enumerate()`, `.flat_map()`

**Consumers** terminate the pipeline and return a concrete value (eager):
`.collect()`, `.sum()`, `.for_each()`, `.find()`, `.count()`

```rust
v.iter()
    .map(|x| x * 2)      // adaptor — still lazy
    .filter(|x| x > &4)  // adaptor — still lazy
    .collect()            // consumer — work happens now
```

---

## The Rust Compiler Collapses the Chain

Because iterators are lazy and the chain is fully known at compile time, the
compiler often inlines and optimizes the entire pipeline into a single tight
loop — equivalent to hand-written imperative code. You get the clarity of a
functional pipeline with zero overhead.

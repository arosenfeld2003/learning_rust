# `.iter()` vs `.into_iter()`

## The Distinction

| Method | Operates on | Yields | Vec after? |
|--------|-------------|--------|------------|
| `.iter()` | `&Vec<T>` (borrow) | `&T` — references | still valid |
| `.into_iter()` | `Vec<T>` (consumes) | `T` — owned values | gone |

`.iter()` borrows the collection and lets you look at elements.
`.into_iter()` consumes the collection and takes ownership of elements.

---

## Two-Step vs Single Pipeline

A two-step approach collects into a `Vec` then re-enters iterator-land:

```rust
let sentence = String::from("the quick brown fox trots");

// Step 1: iterator → Vec
let words: Vec<&str> = sentence
    .split_whitespace()
    .collect();

// Step 2: Vec → iterator → Vec
let words_containing_t: Vec<&str> = words
    .into_iter()
    .filter(|w| w.contains('t'))
    .collect();
```

`into_iter()` is required here because `words` is a `Vec` — it must be
consumed and converted back into an iterator before `.filter()` can operate
on it.

A single pipeline never leaves iterator-land until the final `.collect()`:

```rust
let words_containing_t: Vec<&str> = sentence
    .split_whitespace()
    .filter(|w| w.contains('t'))
    .collect();
```

No intermediate `Vec` is allocated. This is the lazy evaluation benefit —
elements flow through the whole pipeline one at a time.

---

## The Pattern

```
Two-step:   iterator → Vec → iterator → Vec   (two allocations)
Single:     iterator            → Vec          (one allocation)
```

The `.collect()` / `.into_iter()` pair is "leave iterator-land, then
re-enter it." The single pipeline avoids this entirely.

---

## Practical Example: Command-Line Input

```rust
use std::env;

fn main() {
    match env::args().nth(1) {
        Some(sentence) => {
            let words_containing_t: Vec<&str> = sentence
                .split_whitespace()
                .filter(|w| w.contains('t'))
                .collect();

            println!("{:?}", words_containing_t);
        }
        None => {}
    }
}
```

Run it:
```
$ ./program "the quick brown fox trots through the forest"
["the", "trots", "through", "the"]
```

---

## When `.into_iter()` Is the Right Call

When you already have a `Vec` and need to transform or consume it, `.into_iter()`
is correct — you're done with the original and want to process its elements.

```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.into_iter().map(|x| x * 2).collect();
// numbers is gone — its values were moved into the pipeline
```

Use `.iter()` when you want to keep the original around:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
println!("{:?}", numbers);  // still valid
```

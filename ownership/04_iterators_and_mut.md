# Iterators and Mutability

## Example

```rust
fn main() {
    let int_array = [1, 2, 3, 4, 5];
    let mut int_array_it = int_array.iter();

    loop {
        match int_array_it.next() {
            Some(element) => println!("{}", element),
            None => break
        }
    }
}
```

## Why the Iterator Must Be `mut`

`iter()` returns an iterator — an object that tracks a **position** as it walks
through the array. Each call to `.next()` advances that position:

```
int_array_it state after each .next():

initial:   [ 1, 2, 3, 4, 5 ]
            ↑ position = 0

.next() →  [ 1, 2, 3, 4, 5 ]   returns Some(1)
               ↑ position = 1

.next() →  [ 1, 2, 3, 4, 5 ]   returns Some(2)
                  ↑ position = 2
...
.next() →  [ 1, 2, 3, 4, 5 ]   returns None
                           (past the end)
```

Every call to `.next()` changes the internal position — that's a mutation.
So the iterator itself must be `mut`, even though the underlying data is not.

```
int_array        the data       immutable  ← never changes
int_array_it     the cursor     mutable    ← advances each .next()
```

## The Bookmark Mental Model

```
the book (int_array)        → the pages don't change
the bookmark (int_array_it) → its position changes as you read
```

`.next()` moves the bookmark forward. Moving a bookmark is a mutation
of the bookmark, not of the book.

## Key Insight

Rust's `mut` is precise — it's not "this whole thing is mutable," it's
"this specific binding needs to change." The compiler makes you be explicit
about exactly *what* is changing and *why*.

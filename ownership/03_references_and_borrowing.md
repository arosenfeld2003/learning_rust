# References and Borrowing

## The Problem Borrowing Solves

Passing ownership into a function and returning it back out is tedious:

```rust
fn takes_and_gives_back(s: String) -> String {
    s   // return ownership back to caller
}

// worse — if you need a real return value too:
fn calculate_and_keep(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)   // awkward tuple just for ownership housekeeping
}
```

## References

A reference lets you use a value without taking ownership of it.

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}   // s dropped here, but the String it points to is NOT freed

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{} has length {}", s1, len);  // s1 still valid!
}
```

A reference is a pointer to the variable, not to the heap data directly:

```
main() stack frame
──────────────────────────
s1 = ptr→0xA0              ← owns the String

&s1 = ptr→s1               ← reference: points to s1

calculate_length() stack frame
──────────────────────────────
s = ptr→s1                 ← s points to s1, which points to heap

}  ← s dropped here, but s1 is NOT dropped
   reference gone, original value untouched
```

The function **borrows** the value — uses it, hands it back automatically when done.

## Immutable vs Mutable References

```
Immutable reference    &T      read-only
Mutable reference      &mut T  read and write
```

To use a mutable reference:
- The variable must be declared `mut`
- The reference passed must be `&mut`
- The parameter type must be `&mut T`

```rust
fn change(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}
```

## Reference Rules

```
✓  Many immutable references at the same time
        let r1 = &s;
        let r2 = &s;   ← fine

✓  One mutable reference, nothing else
        let r1 = &mut s;   ← fine alone

✗  Two mutable references simultaneously
        let r1 = &mut s;
        let r2 = &mut s;   ← rejected: cannot borrow as mutable more than once

✗  Mutable + immutable simultaneously
        let r1 = &s;
        let r2 = &mut s;   ← rejected: immutable references promise the value won't change
```

**Why:** simultaneous mutable references create data races — unpredictable behavior
when two parts of code write to the same memory without synchronization.
Rust eliminates this entire class of bugs at compile time.

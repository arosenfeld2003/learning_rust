# Panics, Hooks, and catch_unwind

## What is a panic?

A panic is Rust's way of handling unrecoverable errors. When a panic fires, the
thread unwinds and terminates — unless something catches it.

```rust
panic!("Heeelp"); // thread terminates here, unless caught
```

## Two independent mechanisms

### set_hook — controls output

`panic::set_hook` registers a function that runs when a panic fires. It controls
what gets printed. The default hook prints something like:

```
thread 'main' panicked at 'Heeelp', src/main.rs:8:9
```

Replacing it with an empty closure suppresses that output entirely:

```rust
panic::set_hook(Box::new(|_info| {
    // do nothing — suppress default panic output
}));
```

`_info` contains the panic message and location. The underscore prefix means
"I know this exists, I'm deliberately ignoring it."

### catch_unwind — controls propagation

`panic::catch_unwind` runs a closure and catches any panic that fires inside it,
returning a `Result`:

- `Ok(value)` — the closure ran without panicking
- `Err(_)` — a panic occurred

```rust
let result = panic::catch_unwind(|| {
    panic!("Heeelp");
});

match result {
    Ok(res) => res,
    Err(_) => println!("caught panic!"),
}
```

Without `catch_unwind`, the panic would terminate the thread regardless of the hook.
The hook only affects output — it doesn't stop propagation.

## The underscore pattern

`_` in Rust means "something goes here, but I don't care about it." It appears
in destructuring, closure parameters, and match arms:

```rust
let p = (1, 2);
let (_, a) = p; // a = 2, first element discarded

Err(_) => ...   // match any Err, discard its contents
|_info| { }     // closure param received but ignored
```

Using `_` also suppresses Rust's unused variable warning — it's an explicit
signal that the omission is intentional.

## Box::new

`Box::new(...)` heap-allocates a value and returns an owned pointer to it.
Here it's used because `set_hook` needs to own the closure across the program's
lifetime — a stack-allocated closure wouldn't work since the stack frame
that created it may no longer exist when the hook fires.

## Putting it together

```rust
use std::panic;

fn main() {
    // 1. Silence default panic output
    panic::set_hook(Box::new(|_info| {}));

    // 2. Run code that panics, catch the result
    let result = panic::catch_unwind(|| {
        panic!("Heeelp");
    });

    // 3. Handle the outcome — Err always matches here since the closure panics
    match result {
        Ok(res) => res,
        Err(_) => println!("caught panic!"),
    }
}
```

Output: `caught panic!`

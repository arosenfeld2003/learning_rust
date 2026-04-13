# Ownership Rules

Ownership is Rust's way of tracking, **at compile time**, exactly who is responsible
for heap memory — so it gets freed exactly once, at exactly the right time.

## The Three Rules

```
Rule 1: Each value has an owner
        let s = String::from("hello");
                ─────────────────────
                s owns this value

Rule 2: Only one owner at a time
        let s2 = s;   ← ownership MOVES to s2
                          s is no longer valid

Rule 3: When the owner goes out of scope, the value is dropped
        {
            let s = String::from("hello");
            // ... use s ...
        }  ← s goes out of scope, memory freed here
               automatically, no free() needed
```

## Rule 3: Deterministic Drop

The closing brace isn't just syntax — the compiler inserts a call to `drop()` there.
Memory is freed **deterministically**, at a known point, with no garbage collector needed.

```
{                        ← s comes into scope
    let s = String::from("hello");

}                        ← compiler calls drop(s) here
                           heap memory freed
                           stack frame cleaned up
```

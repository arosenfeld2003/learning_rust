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

## Rule 2: Move vs Copy

Assignment behavior depends on whether a type implements the `Copy` trait.

**Types that implement `Copy`** (stack-only, fixed size):
- Integers (`i32`, `u64`, etc.), `bool`, `f64`, `char`
- Tuples of Copy types
- Assignment **duplicates the bits** — both variables remain valid

**Types that do NOT implement `Copy`** (heap-allocated, variable size):
- `String`, `Vec`, etc.
- Assignment **moves** ownership — original variable is invalidated

```
i32 (Copy):                         String (not Copy):
───────────────────────             ───────────────────────────────────
let x = 5;                          let s1 = String::from("hello");
let y = x;                          let s2 = s1;

stack: [ x=5, y=5 ]                 stack: [ s2=ptr→0xA0 ]
both valid ✓                        heap:  [ 0xA0: "hello" ]
                                    s1 invalidated ✗
```

**Rust never implicitly copies heap data.** If you need both variables, use `.clone()`:

```
Need both variables?   → s2 = s1.clone()   (two heap allocations)
Done with original?    → let s2 = s1       (move, no extra allocation)
Just need to read?     → borrow            (coming soon)
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

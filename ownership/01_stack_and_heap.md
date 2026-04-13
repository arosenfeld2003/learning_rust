# Stack and Heap

## The Stack

- LIFO (Last In, First Out) — like a pile of plates
- Push data on, pop data off
- All stored data must have a **known, fixed size at compile time**
- Fast to access — the CPU knows where to look

## The Heap

- For data with an unknown size, or a size that may change
- You request a specific amount of space from the memory allocator
- The allocator finds a free spot, marks it as used, and returns a **pointer**
- The pointer is a known, fixed size — so it lives on the stack
- To get the actual data, you follow the pointer

```
STACK                          HEAP
─────────────────              ──────────────────────────────
│                │             │  addr  │  data              │
│                │             ├────────┼────────────────────┤
│                │             │  0x01  │  ...               │
│  ptr → 0xA0   │ ──────────► │  ...   │  ...               │
│  len: 5        │             │  0xA0  │  "hello"  ◄──────  │
│  cap: 5        │             │  ...   │  ...               │
│                │             │  ...   │  (free)            │
│                │             │  ...   │  (free)            │
│                │             ──────────────────────────────
│  i32: 42       │
│  bool: true    │
─────────────────
  ↑ grows/shrinks
    at runtime
    (known sizes)
```

## Why Heap Access is Slower

```
STACK ACCESS                    HEAP ACCESS
─────────────────               ─────────────────
 1. read value                   1. read pointer     (stack)
    ✓ done                       2. follow pointer → jump to heap address
                                 3. read value       (heap)
                                    ✓ done
```

The heap requires an extra indirection step. It can also cause a **cache miss** —
the CPU may have to fetch data from a distant memory address not already in cache.

```
CPU Cache (fast, small)
┌─────────────────────────────────┐
│  stack values likely here  ✓   │
│  heap values: maybe, maybe not  │ ← cache miss = slow
└─────────────────────────────────┘
```

## The Core Problem Ownership Solves

Heap data can **outlive the function that created it**:

```rust
fn make_string() -> String {
    let s = String::from("hello");  // allocated on heap
    s                               // returned — outlives the function
}
```

```
make_string() stack frame        caller's stack frame
─────────────────────            ─────────────────────
│  ptr → 0xA0        │    →     │  ptr → 0xA0        │
─────────────────────            ─────────────────────
      (popped)                         (still alive)

HEAP: [ 0xA0: "hello" ]         ← who is responsible for this now?
```

Without a system to track responsibility, you get:
- **Memory leaks** — freed too late, or never
- **Double free** — freed twice → crash
- **Use after free** — freed too early → undefined behavior

In C, the programmer manages this manually at runtime.
In Rust, the **compiler** manages it at compile time — mistakes won't compile.

```
C / C++                          Rust
─────────────────────────        ─────────────────────────
programmer manages memory        compiler manages memory
mistakes caught at runtime       mistakes caught at compile time
  → segfault                       → won't compile
  → leak                           → won't compile
  → undefined behavior             → won't compile
  → maybe never caught             → always caught
```

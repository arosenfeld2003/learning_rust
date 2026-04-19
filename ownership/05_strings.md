# Strings: `&str` vs `&String`

## The Four Types

**`String`** — an owned, heap-allocated string. It controls the memory and is
responsible for freeing it when it goes out of scope.

```rust
let s = String::from("hello");  // s owns the data on the heap
```

**`str`** — a string slice: a sequence of UTF-8 bytes somewhere in memory.
It is **unsized** — its size is not known at compile time. You can never hold
a `str` directly; it must always be behind a pointer.

**`&str`** — a reference to a string slice. This is a **fat pointer**: it
carries two pieces of information:
- a memory address (where the bytes start)
- a length (how many bytes)

The bytes it points to can live anywhere:

```rust
let s: &str = "hello";        // points into the binary (static memory)
let s: &str = &some_string;   // points into a String's heap memory
```

**`&String`** — a reference to an owned `String`. This is a **thin pointer**:
just a memory address pointing to the `String` struct (which itself points to
heap data).

```rust
let s = String::from("hello");
let r: &String = &s;          // reference to the String struct
```

---

## Fat vs Thin Pointers

A regular reference (`&T`) is a thin pointer — just an address.

A slice reference (`&[T]`, `&str`) is a fat pointer — an address plus a length.
This extra length field is necessary because the size of the slice isn't known
at compile time.

```
&String  →  [ address ]                  (thin: 1 word)
&str     →  [ address | length ]         (fat: 2 words)
```

---

## Deref Coercion: `&String → &str`

`&String` can always be used where `&str` is expected. The compiler silently
converts it by reading the address and length out of the `String` struct.

```
&String  ──deref coercion──▶  &str
```

The reverse is not true — you cannot use `&str` where `&String` is required.

---

## Why Prefer `&str` in Function Signatures

Because `&String` coerces to `&str`, a function taking `&str` accepts both:

```rust
fn greet(name: &str) {}

greet(&some_string);   // &String coerces to &str ✓
greet("hello");        // string literal is already &str ✓
```

A function taking `&String` only accepts `&String`:

```rust
fn greet(name: &String) {}

greet(&some_string);   // works ✓
greet("hello");        // fails — "hello" is &str, not &String ✗
```

**Rule:** prefer `&str` for function parameters. It is strictly more flexible.

---

## `.clone()` Behavior

| Expression        | Type of result | What happens                        |
|-------------------|----------------|-------------------------------------|
| `some_str.clone()`    | `&str`     | copies the fat pointer (no allocation) |
| `some_string.clone()` | `String`   | copies the heap data (allocates)    |

Cloning a `&str` is cheap — you're just copying two words (address + length).
Cloning a `String` allocates a new buffer on the heap.

---

## String Concatenation and `+`

The `+` operator is defined as `String + &str → String`. It consumes the left
`String` and appends the right `&str` to it.

```rust
let s1 = String::from("Hello");
let s2 = String::from(" world");

let result = s1 + &s2;   // s1 is moved (consumed), s2 is borrowed
// s1 is no longer valid here
```

To keep the original, clone it first:

```rust
let result = s1.clone() + &s2;  // s1 still valid
```

---

## Key Distinctions

```
String      owns the data          heap-allocated       can grow/shrink
str         the raw bytes          unsized, no owner    always behind a pointer
&str        view into bytes        fat pointer          read-only, any lifetime
&String     reference to owner     thin pointer         read-only, tied to String
```

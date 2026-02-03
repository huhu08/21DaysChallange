# Day 15: Lifetimes

## What Will We Learn Today?
- Why lifetimes exist
- Lifetime annotations
- Lifetime elision rules
- Lifetimes in structs
- Static lifetime
- Combining lifetimes with generics

## Why Lifetimes?

Lifetimes prevent dangling references - references to data that no longer exists.

```rust
// This won't compile - r would be a dangling reference
fn main() {
    let r;
    {
        let x = 5;
        r = &x;  // x is dropped at end of this scope
    }
    println!("{}", r);  // r references freed memory!
}
```

## Lifetime Annotations

When a function returns a reference, Rust needs to know how long that reference is valid.

```rust
// Both inputs and output share lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The annotation `'a` says: "the returned reference will be valid as long as both inputs are valid."

## Lifetime Elision Rules

The compiler can often infer lifetimes automatically:

1. Each reference parameter gets its own lifetime
2. If there's exactly one input lifetime, output gets that lifetime
3. If there's `&self` or `&mut self`, output gets self's lifetime

```rust
// Compiler infers: fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    // ...
}
```

## Lifetimes in Structs

Structs holding references need lifetime annotations:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce(&self, msg: &str) -> &str {
        println!("{}", msg);
        self.part  // Returns reference with lifetime 'a
    }
}
```

## Static Lifetime

The `'static` lifetime means the reference lives for the entire program:

```rust
let s: &'static str = "I'm a string literal";
// String literals are stored in binary, so they're always valid
```

## Combining with Generics

```rust
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("{}", ann);
    if x.len() > y.len() { x } else { y }
}
```

## Common Patterns

### Returning One of Two References
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

### Struct with Reference
```rust
struct Parser<'a> {
    input: &'a str,
}
```

### Lifetime Bounds
```rust
struct Ref<'a, T: 'a>(&'a T);
// T: 'a means T must live at least as long as 'a
```

## Run the Examples

```bash
cd day-15
cargo run
```

## Homework

### Homework 1: Conditional Longer
Create a function that returns the longer string only if they start with the same character:

```rust
fn longer_if_same_start<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Return Some(longer) if first chars match
    // Return None otherwise
}
```

### Homework 2: Tokenizer
Create a tokenizer that holds a reference to text and iterates over words:

```rust
struct Tokenizer<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(text: &'a str) -> Self { /* ... */ }
    fn next_word(&mut self) -> Option<&'a str> { /* ... */ }
}
```

### Homework 3: Cache with Reference
Create a cache struct that stores a reference and computed value:

```rust
struct Cache<'a, T, R> {
    data: &'a T,
    computed: Option<R>,
}

impl<'a, T, R> Cache<'a, T, R> {
    fn new(data: &'a T) -> Self { /* ... */ }
    fn get_or_compute<F>(&mut self, f: F) -> &R 
    where F: FnOnce(&T) -> R { /* ... */ }
}
```

---

[← Day 14: Week 2 Project](../day-14/README.md) | [Home](../README.md) | [Day 16: Closures →](../day-16/README.md)

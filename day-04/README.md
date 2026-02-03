# Day 4: Ownership

## What Will We Learn Today?
- Ownership rules
- Move semantics
- Clone and Copy
- Ownership in functions

## Ownership Rules

1. Each value has one owner
2. Only one owner at a time
3. Value dropped when owner goes out of scope

## Move vs Clone

```rust
let s1 = String::from("hello");
let s2 = s1;        // Move - s1 invalid
let s3 = s2.clone(); // Clone - both valid
```

## Copy Trait

Stack-only types (integers, bool, char) implement Copy:

```rust
let x = 5;
let y = x;  // Copy - both valid
```

## Run the Examples

```bash
cd day-04
cargo run
```

## Homework

### Homework 1: Reverse String
```rust
fn reverse_string(s: String) -> String
```

### Homework 2: Experiment with Moves
Observe compiler errors when using moved values.

### Homework 3: Sum and Return Vector
```rust
fn sum_and_return(v: Vec<i32>) -> (Vec<i32>, i32)
```

---

[← Previous Day](../day-03/README.md) | [Next Day: Borrowing →](../day-05/README.md)

# Day 5: Borrowing and References

## What Will We Learn Today?
- References and borrowing
- Mutable references
- Borrowing rules

## References

```rust
fn len(s: &String) -> usize { s.len() }

let s = String::from("hello");
let l = len(&s);  // s still valid
```

## Mutable References

```rust
fn change(s: &mut String) {
    s.push_str(" world");
}
```

## Borrowing Rules

1. Either ONE mutable reference OR any number of immutable references
2. References must always be valid

## Run the Examples

```bash
cd day-05
cargo run
```

## Homework

### Homework 1: Add Ten to Vector
```rust
fn add_ten(v: &mut Vec<i32>)
```

### Homework 2: Find Longest
```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str
```

### Homework 3: Swap Values
```rust
fn swap(a: &mut i32, b: &mut i32)
```

---

[← Previous Day](../day-04/README.md) | [Next Day: Week 1 Project →](../day-06/README.md)

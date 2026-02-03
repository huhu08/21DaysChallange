# Day 3: Control Flow

## What Will We Learn Today?
- if/else expressions
- loop, while, for loops
- break, continue, loop labels

## Conditionals

```rust
if score >= 90 { "A" } else { "B" }
```

## Loops

```rust
// Infinite loop with break
loop { if done { break; } }

// While loop
while n > 0 { n -= 1; }

// For loop
for i in 1..=5 { println!("{}", i); }
```

## Loop Labels

```rust
'outer: loop {
    loop { break 'outer; }
}
```

## Run the Examples

```bash
cd day-03
cargo run
```

## Homework

### Homework 1: Print Even Numbers
```rust
fn print_even_numbers(n: u32)
```

### Homework 2: Sum Calculator
```rust
fn sum_to_n(n: u32) -> u32
```

### Homework 3: Guessing Game
Create a loop that keeps asking for guesses until correct.

---

[← Previous Day](../day-02/README.md) | [Next Day: Ownership →](../day-04/README.md)

# Day 2: Functions

## What Will We Learn Today?
- Function definitions
- Parameters and return values
- Expressions vs statements
- Recursive functions

## Functions

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}
```

## Expressions vs Statements

```rust
let y = {
    let x = 3;
    x + 1  // Expression - returns value
};
```

## Run the Examples

```bash
cd day-02
cargo run
```

## Homework

### Homework 1: Rectangle Area
```rust
fn rectangle_area(width: f64, height: f64) -> f64
```

### Homework 2: Prime Checker
```rust
fn is_prime(n: u32) -> bool
```

### Homework 3: Temperature Converter
```rust
fn celsius_to_fahrenheit(celsius: f64) -> f64
// Formula: F = C * 9/5 + 32
```

---

[← Previous Day](../day-01/README.md) | [Next Day: Control Flow →](../day-03/README.md)

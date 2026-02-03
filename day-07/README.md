# Day 7: Week 1 Project - Calculator

## Overview

Congratulations on completing Week 1! Today you'll build a command-line calculator that combines everything you've learned so far:

- **Variables & Data Types** (Day 1-2)
- **Functions** (Day 3)
- **Control Flow** (Day 4)
- **Ownership** (Day 5)
- **Borrowing & References** (Day 6)
- **Slices** (Day 6)

## Project Description

Build an interactive command-line calculator with the following features:

### Core Features (Implemented)
1. **Basic Operations**: Addition, Subtraction, Multiplication, Division
2. **Power Calculation**: Calculate base^exponent
3. **Factorial**: Calculate n!
4. **Temperature Converter**: Celsius ↔ Fahrenheit

### Concepts Applied

#### Ownership & Borrowing
```rust
fn get_number(prompt: &str) -> f64 {
    // prompt is borrowed (&str)
    // Returns owned f64
}
```

#### Option for Safe Division
```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
```

#### Pattern Matching
```rust
match choice.trim() {
    "1" => basic_operations(),
    "2" => power_calculation(),
    _ => println!("Invalid option"),
}
```

## Running the Project

```bash
cd day-07
cargo run
```

## Homework

Extend the calculator with these additional features:

### Exercise 1: History Feature
Store calculation history using a `Vec<String>` and display the last 10 calculations when requested.

```rust
// Hint: Create a mutable Vec to store history
let mut history: Vec<String> = Vec::new();
history.push(format!("{} + {} = {}", a, b, result));
```

### Exercise 2: More Mathematical Operations
Add the following operations:
- Square root (`f64::sqrt()`)
- Modulo/Remainder (`%`)
- Absolute value (`f64::abs()`)
- Percentage calculation

### Exercise 3: Input Validation with Slices
Create a function that validates user input using string slices:

```rust
fn validate_input(input: &str) -> bool {
    // Check if input contains only valid characters
    // Use slice methods like chars(), is_numeric(), etc.
}
```

### Exercise 4: Memory Feature
Implement calculator memory functions:
- **M+**: Add current result to memory
- **M-**: Subtract current result from memory
- **MR**: Recall memory value
- **MC**: Clear memory

```rust
struct Calculator {
    memory: f64,
    history: Vec<String>,
}
```

### Exercise 5: Expression Parsing (Advanced)
Parse and evaluate simple mathematical expressions:

```rust
// Input: "2 + 3 * 4"
// Output: 14 (following order of operations)

fn parse_expression(expr: &str) -> Option<f64> {
    // Use string slices to tokenize
    // Handle operator precedence
}
```

## Expected Output

```
=== Day 7: Week 1 Project - Calculator ===

Welcome to the Rust Calculator!
This project combines: Variables, Functions, Control Flow, Ownership, Borrowing, and Slices

--- Calculator Menu ---
1. Basic Operations (+, -, *, /)
2. Power Calculation
3. Factorial
4. Temperature Converter
5. Exit

Choose an option (1-5): 1

--- Basic Operations ---
Enter first number: 10
Enter second number: 5

Select operation:
1. Addition (+)
2. Subtraction (-)
3. Multiplication (*)
4. Division (/)
1
Result: 15
```

## Week 1 Summary

| Day | Topic | Key Concepts |
|-----|-------|--------------|
| 0 | Introduction | Rust setup, Hello World, Cargo basics |
| 1 | Variables | let, mut, const, shadowing |
| 2 | Data Types | Scalars, compounds, type inference |
| 3 | Functions | Parameters, return values, expressions |
| 4 | Control Flow | if/else, loops, match |
| 5 | Ownership | Move semantics, Copy trait |
| 6 | Borrowing & Slices | References, string slices, array slices |
| 7 | **Project** | Calculator combining all concepts |

## Navigation

[← Day 6: Slices](../day-06/README.md) | [Home](../README.md) | [Day 8: Structs →](../day-08/README.md)

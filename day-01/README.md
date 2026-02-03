# Day 1: Variables and Data Types

## What Will We Learn Today?
- Immutable and mutable variables
- Constants and shadowing
- Primitive data types
- Compound types (tuples, arrays)

## Variables

```rust
let x = 5;           // Immutable
let mut y = 10;      // Mutable
const MAX: u32 = 100; // Constant
```

## Shadowing

```rust
let x = 5;
let x = x + 1;           // Shadowing
let x = "now a string";  // Can change type
```

## Data Types

| Type | Examples |
|------|----------|
| Integers | i8, i32, u64, isize |
| Floats | f32, f64 |
| Boolean | bool |
| Character | char |
| Tuple | (i32, f64, char) |
| Array | [i32; 5] |

## Run the Examples

```bash
cd day-01
cargo run
```

## Homework

### Homework 1: Create Variables of Each Type
Create and print variables of each data type.

### Homework 2: Practice Shadowing
Shadow a variable multiple times with different values/types.

### Homework 3: Mutable Counter
Create a mutable counter and increment it 5 times in a loop.

---

[← Previous Day](../day-00/README.md) | [Next Day: Functions →](../day-02/README.md)

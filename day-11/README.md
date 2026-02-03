# Day 11: Collections

## What Will We Learn Today?
- Vectors (`Vec<T>`)
- Strings (`String`)
- Hash Maps (`HashMap<K, V>`)

## Vectors

Vectors store multiple values of the same type in contiguous memory.

### Creating Vectors

```rust
let v1: Vec<i32> = Vec::new();      // Empty vector
let v2 = vec![1, 2, 3, 4, 5];       // With initial values
```

### Modifying Vectors

```rust
let mut v = Vec::new();
v.push(1);           // Add element
v.pop();             // Remove and return last element
v.insert(0, 10);     // Insert at index
v.remove(0);         // Remove at index
```

### Accessing Elements

```rust
let v = vec![10, 20, 30];

// Using index (panics if out of bounds)
let third = &v[2];

// Using get (returns Option - safe)
match v.get(2) {
    Some(value) => println!("{}", value),
    None => println!("No element"),
}
```

### Iterating

```rust
let v = vec![1, 2, 3];

// Read-only
for i in &v {
    println!("{}", i);
}

// Mutable
for i in &mut v {
    *i *= 2;
}
```

## Strings

Strings in Rust are UTF-8 encoded and growable.

### Creating Strings

```rust
let s1 = String::new();
let s2 = String::from("hello");
let s3 = "world".to_string();
```

### Modifying Strings

```rust
let mut s = String::from("foo");
s.push_str("bar");  // Append string slice
s.push('!');        // Append single char

// Concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("World!");
let s3 = s1 + &s2;  // s1 is moved

// format! macro (doesn't take ownership)
let s = format!("{}-{}", s1, s2);
```

## Hash Maps

Hash maps store key-value pairs with O(1) access.

### Creating and Using

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// Access
let score = scores.get("Blue");  // Returns Option<&V>

// Iterate
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### Updating

```rust
// Overwrite
scores.insert(String::from("Blue"), 25);

// Only insert if key doesn't exist
scores.entry(String::from("Red")).or_insert(50);

// Update based on old value
let count = map.entry(key).or_insert(0);
*count += 1;
```

## Run the Examples

```bash
cd day-11
cargo run
```

## Homework

### Homework 1: Analyze Numbers
Create a function that analyzes a vector of numbers:

```rust
fn analyze(numbers: &Vec<i32>) -> (i32, i32, i32, f64) {
    // Returns: (min, max, sum, average)
}
```

### Homework 2: Character Frequency
Create a function that counts character frequency:

```rust
fn char_frequency(s: &str) -> HashMap<char, i32> {
    // Count how many times each character appears
}
```

### Homework 3: Inventory System
Create an inventory management system:

```rust
struct Inventory {
    items: HashMap<String, u32>,
}

impl Inventory {
    fn new() -> Self { /* ... */ }
    fn add_item(&mut self, name: &str, quantity: u32) { /* ... */ }
    fn remove_item(&mut self, name: &str, quantity: u32) -> bool { /* ... */ }
    fn get_quantity(&self, name: &str) -> u32 { /* ... */ }
}
```

---

[← Day 10: Error Handling](../day-10/README.md) | [Home](../README.md) | [Day 12: Generics →](../day-12/README.md)

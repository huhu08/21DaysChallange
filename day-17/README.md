# Day 17: Iterators

## What Will We Learn Today?
- Iterator trait and next()
- Iterator types (iter, iter_mut, into_iter)
- Iterator adaptors (map, filter, etc.)
- Consuming adaptors (sum, collect, fold)
- Creating custom iterators

## The Iterator Trait

All iterators implement the `Iterator` trait:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // Many default methods...
}
```

## Creating Iterators

```rust
let v = vec![1, 2, 3];

// iter() - borrows, yields &T
for val in v.iter() { }

// iter_mut() - borrows mutably, yields &mut T
for val in v.iter_mut() { *val *= 2; }

// into_iter() - takes ownership, yields T
for val in v.into_iter() { }
```

## Iterator Adaptors

Adaptors transform iterators lazily (nothing happens until consumed):

### map - Transform Elements
```rust
let doubled: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect();
// [2, 4, 6]
```

### filter - Keep Matching Elements
```rust
let evens: Vec<i32> = (1..=10)
    .filter(|x| x % 2 == 0)
    .collect();
// [2, 4, 6, 8, 10]
```

### Chaining
```rust
let result: Vec<i32> = (1..=10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .collect();
// [4, 16, 36, 64, 100]
```

## Consuming Adaptors

These consume the iterator to produce a value:

```rust
let nums = vec![1, 2, 3, 4, 5];

nums.iter().sum::<i32>()        // 15
nums.iter().product::<i32>()    // 120
nums.iter().max()               // Some(&5)
nums.iter().min()               // Some(&1)
nums.iter().count()             // 5
```

## fold - Flexible Reduction

```rust
let sum = nums.iter().fold(0, |acc, x| acc + x);
let product = nums.iter().fold(1, |acc, x| acc * x);
```

## Search Methods

```rust
nums.iter().find(|&&x| x > 3)   // Some(&4)
nums.iter().any(|&x| x > 4)     // true
nums.iter().all(|&x| x > 0)     // true
```

## Other Useful Adaptors

```rust
// take, skip
(1..10).take(3)          // 1, 2, 3
(1..10).skip(7)          // 8, 9

// enumerate - add indices
for (i, val) in v.iter().enumerate() { }

// zip - combine iterators
let pairs: Vec<_> = a.iter().zip(b.iter()).collect();

// flatten - flatten nested iterators
vec![vec![1, 2], vec![3]].into_iter().flatten()  // 1, 2, 3
```

## Creating Custom Iterators

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

## Run the Examples

```bash
cd day-17
cargo run
```

## Homework

### Homework 1: Range Iterator
Create your own range iterator:

```rust
struct MyRange {
    current: i32,
    end: i32,
}

impl MyRange {
    fn new(start: i32, end: i32) -> Self { /* ... */ }
}

impl Iterator for MyRange {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> { /* ... */ }
}
```

### Homework 2: Word Frequency Counter
Use iterators to count word frequency:

```rust
use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        // filter, map, and collect into HashMap
}
```

### Homework 3: Pairs Iterator
Create an iterator that yields consecutive pairs:

```rust
struct Pairs<I: Iterator> {
    iter: I,
    previous: Option<I::Item>,
}

// [1, 2, 3, 4] -> [(1, 2), (2, 3), (3, 4)]
```

---

[← Day 16: Closures](../day-16/README.md) | [Home](../README.md) | [Day 18: Smart Pointers →](../day-18/README.md)

# Day 12: Generics

## What Will We Learn Today?
- Generic functions
- Generic structs
- Generic enums
- Generic methods
- Trait bounds on generics

## Why Generics?

Generics allow you to write flexible, reusable code without sacrificing performance. The compiler generates specific code for each type used (monomorphization).

## Generic Functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Works with any type that implements PartialOrd
let numbers = vec![34, 50, 25, 100, 65];
let result = largest(&numbers);  // Works with i32

let chars = vec!['y', 'm', 'a', 'q'];
let result = largest(&chars);    // Works with char
```

## Generic Structs

```rust
// Single type parameter
struct Point<T> {
    x: T,
    y: T,  // x and y must be same type
}

// Multiple type parameters
struct MixedPoint<T, U> {
    x: T,
    y: U,  // x and y can be different types
}

let int_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.0, y: 4.0 };
let mixed = MixedPoint { x: 5, y: 4.0 };
```

## Generic Methods

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Methods only for specific types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

## Generic Enums

```rust
// Like Option<T>
enum MyOption<T> {
    Some(T),
    None,
}

// Like Result<T, E>
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}
```

## Trait Bounds

Restrict generics to types implementing certain traits:

```rust
use std::fmt::Display;

// T must implement Display and PartialOrd
fn print_largest<T: Display + PartialOrd>(list: &[T]) {
    let largest = largest(list);
    println!("The largest is {}", largest);
}

// Alternative syntax with where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

## Run the Examples

```bash
cd day-12
cargo run
```

## Homework

### Homework 1: Generic Queue
Create a generic `Queue<T>` data structure:

```rust
struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self { /* ... */ }
    fn enqueue(&mut self, item: T) { /* ... */ }
    fn dequeue(&mut self) -> Option<T> { /* ... */ }
    fn peek(&self) -> Option<&T> { /* ... */ }
    fn is_empty(&self) -> bool { /* ... */ }
    fn len(&self) -> usize { /* ... */ }
}
```

### Homework 2: Pair with Swap
Create a `Pair<T, U>` struct with a swap method:

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self { /* ... */ }
    fn swap(self) -> Pair<U, T> { /* ... */ }
}
```

### Homework 3: Generic Minimum Function
Create a generic function that finds the minimum element:

```rust
fn minimum<T: Ord>(list: &[T]) -> Option<&T> {
    // Return None if list is empty
    // Return Some(&min) otherwise
}
```

---

[← Day 11: Collections](../day-11/README.md) | [Home](../README.md) | [Day 13: Traits →](../day-13/README.md)

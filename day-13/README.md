# Day 13: Traits

## What Will We Learn Today?
- Defining and implementing traits
- Default implementations
- Traits as parameters
- Trait bounds
- Derivable traits
- Trait objects

## What Are Traits?

Traits define shared behavior. They're similar to interfaces in other languages.

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

## Implementing Traits

```rust
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
```

## Default Implementations

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Types can use default or override it
impl Summary for NewsArticle {}  // Uses default
```

## Traits as Parameters

```rust
// impl Trait syntax
fn notify(item: &impl Summary) {
    println!("Breaking: {}", item.summarize());
}

// Trait bound syntax (equivalent)
fn notify<T: Summary>(item: &T) {
    println!("Breaking: {}", item.summarize());
}
```

## Multiple Trait Bounds

```rust
// Using + syntax
fn process(item: &(impl Display + Debug)) {
    println!("{}", item);
}

// Using where clause
fn process<T>(item: &T)
where
    T: Display + Debug + Clone,
{
    println!("{}", item);
}
```

## Derivable Traits

Common traits that can be automatically implemented:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
```

## Implementing Display

```rust
use std::fmt::{self, Display};

struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}
```

## Trait Objects (Dynamic Dispatch)

```rust
trait Shape {
    fn area(&self) -> f64;
}

// Vec can hold different types implementing Shape
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle { width: 4.0, height: 6.0 }),
];
```

## Returning Traits

```rust
fn create_summarizable() -> impl Summary {
    Tweet {
        username: String::from("rust"),
        content: String::from("Hello!"),
    }
}
```

## Run the Examples

```bash
cd day-13
cargo run
```

## Homework

### Homework 1: Drawable Trait
Create a `Drawable` trait and implement it for different shapes:

```rust
trait Drawable {
    fn draw(&self);
    fn bounding_box(&self) -> (f64, f64, f64, f64);  // x, y, width, height
}

// Implement for Circle, Rectangle, Line
```

### Homework 2: Comparable Trait
Create a custom comparison trait:

```rust
use std::cmp::Ordering;

trait Comparable {
    fn compare(&self, other: &Self) -> Ordering;
}

// Implement for a Product struct (compare by price)
struct Product {
    name: String,
    price: f64,
}
```

### Homework 3: Generic Function with Multiple Bounds
Create a generic function that requires multiple traits:

```rust
fn process_and_display<T>(item: T)
where
    T: Clone + Debug + Display,
{
    let cloned = item.clone();
    println!("Display: {}", cloned);
    println!("Debug: {:?}", cloned);
}
```

---

[← Day 12: Generics](../day-12/README.md) | [Home](../README.md) | [Day 14: Week 2 Project →](../day-14/README.md)

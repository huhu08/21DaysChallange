# Day 8: Structs

## What Will We Learn Today?
- Defining and instantiating structs
- Tuple structs and unit-like structs
- Struct update syntax
- Methods and associated functions
- Multiple impl blocks

## Defining Structs

Structs let you create custom data types by grouping related values together.

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
```

## Creating Instances

```rust
let user1 = User {
    username: String::from("alice123"),
    email: String::from("alice@example.com"),
    active: true,
    sign_in_count: 1,
};
```

## Mutable Structs

The entire instance must be mutable; Rust doesn't allow marking individual fields as mutable.

```rust
let mut user = User { /* ... */ };
user.email = String::from("new@example.com");
```

## Struct Update Syntax

Create a new instance using values from another instance:

```rust
let user2 = User {
    email: String::from("new@example.com"),
    ..user1  // Use remaining fields from user1
};
```

## Tuple Structs

Structs without named fields:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

## Unit-like Structs

Structs without any fields:

```rust
struct AlwaysEqual;
let subject = AlwaysEqual;
```

## Methods

Methods are functions defined within the context of a struct:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method: first parameter is always self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

## Associated Functions

Functions in impl blocks that don't take self:

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Called with ::
let sq = Rectangle::square(10);
```

## Run the Examples

```bash
cd day-08
cargo run
```

## Homework

### Homework 1: Circle Struct
Create a `Circle` struct with:
- Field: `radius` (f64)
- Methods: `area()`, `circumference()`, `diameter()`
- Associated function: `new(radius: f64) -> Circle`

```rust
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle { /* ... */ }
    fn area(&self) -> f64 { /* ... */ }
    fn circumference(&self) -> f64 { /* ... */ }
    fn diameter(&self) -> f64 { /* ... */ }
}
```

### Homework 2: Person Struct
Create a `Person` struct with:
- Fields: `name`, `age`, `email`
- Methods:
  - `birthday(&mut self)` - increments age
  - `update_email(&mut self, new_email: String)`
  - `introduce(&self)` - prints introduction

### Homework 3: BankAccount Struct
Create a `BankAccount` struct with:
- Fields: `account_number`, `holder_name`, `balance`
- Methods:
  - `deposit(&mut self, amount: f64)`
  - `withdraw(&mut self, amount: f64) -> bool`
  - `transfer(&mut self, other: &mut BankAccount, amount: f64) -> bool`

---

[← Day 7: Week 1 Project](../day-07/README.md) | [Home](../README.md) | [Day 9: Enums →](../day-09/README.md)

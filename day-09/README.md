# Day 9: Enums and Pattern Matching

## What Will We Learn Today?
- Defining enums
- Enums with data
- The Option enum
- Pattern matching with match
- if let syntax

## Defining Enums

Enums allow you to define a type by enumerating its possible variants:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let dir = Direction::North;
```

## Enums with Data

Each variant can hold different types and amounts of data:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

## Different Data Types per Variant

```rust
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields
    Write(String),              // Single String
    ChangeColor(u8, u8, u8),    // Three values
}
```

## The Option Enum

Rust's way of handling nullable values:

```rust
enum Option<T> {
    None,
    Some(T),
}

let some_number: Option<i32> = Some(5);
let no_number: Option<i32> = None;
```

## Pattern Matching with match

```rust
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Catch-all Patterns

```rust
match dice_roll {
    3 => println!("Fancy hat!"),
    7 => println!("Remove fancy hat!"),
    other => println!("Move {} spaces", other),
    // or use _ to ignore: _ => ()
}
```

## if let Syntax

Concise way to handle a single pattern:

```rust
let config_max = Some(3u8);

// Instead of match
if let Some(max) = config_max {
    println!("Maximum: {}", max);
}
```

## Methods on Enums

```rust
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Write(text) => println!("{}", text),
            // ...
        }
    }
}
```

## Run the Examples

```bash
cd day-09
cargo run
```

## Homework

### Homework 1: TrafficLight Enum
Create a `TrafficLight` enum with variants: Red, Yellow, Green.
Implement a `duration` function that returns how long each light lasts:
- Red: 60 seconds
- Yellow: 5 seconds
- Green: 45 seconds

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn duration(light: &TrafficLight) -> u32 {
    // Your implementation
}
```

### Homework 2: Shape Enum with Area
Create a `Shape` enum with variants:
- `Circle(f64)` - radius
- `Rectangle(f64, f64)` - width, height
- `Triangle(f64, f64)` - base, height

Implement an `area` function for each variant.

### Homework 3: WebEvent Enum
Create a `WebEvent` enum with variants:
- `PageLoad`
- `KeyPress(char)`
- `Click { x: i64, y: i64 }`
- `Paste(String)`

Create a function that describes what happened for each event.

---

[← Day 8: Structs](../day-08/README.md) | [Home](../README.md) | [Day 10: Error Handling →](../day-10/README.md)

# Day 20: Macros

## What Will We Learn Today?
- Declarative macros (macro_rules!)
- Macro patterns and repetition
- Built-in macros
- Procedural macros overview

## What Are Macros?

Macros are a form of metaprogramming - code that writes code. Unlike functions, macros:
- Can take a variable number of arguments
- Are expanded at compile time
- Can generate code based on patterns

## Declarative Macros (macro_rules!)

### Simple Macro

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

say_hello!();  // Prints: Hello!
```

### Macro with Arguments

```rust
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

greet!("Alice");  // Prints: Hello, Alice!
```

### Multiple Patterns

```rust
macro_rules! calculate {
    ($a:expr + $b:expr) => { $a + $b };
    ($a:expr - $b:expr) => { $a - $b };
    ($a:expr * $b:expr) => { $a * $b };
}

let sum = calculate!(5 + 3);      // 8
let diff = calculate!(10 - 4);    // 6
```

## Repetition

Use `$( ... ),*` for zero or more repetitions:

```rust
macro_rules! create_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

let v = create_vec![1, 2, 3, 4, 5];
```

## Designators

| Designator | Matches |
|------------|---------|
| `expr` | Expression |
| `ident` | Identifier |
| `ty` | Type |
| `pat` | Pattern |
| `block` | Block of code |
| `stmt` | Statement |
| `item` | Item (fn, struct, etc.) |
| `tt` | Token tree |

## Common Macro Patterns

### HashMap Literal

```rust
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

let scores = hashmap! {
    "Alice" => 100,
    "Bob" => 85
};
```

### Debug Print (Conditional)

```rust
macro_rules! debug_print {
    ($msg:expr) => {
        #[cfg(debug_assertions)]
        println!("DEBUG: {}", $msg);
    };
}
```

### Measure Time

```rust
macro_rules! measure_time {
    ($name:expr, $block:block) => {
        {
            let start = std::time::Instant::now();
            let result = $block;
            println!("{} took {:?}", $name, start.elapsed());
            result
        }
    };
}
```

## Procedural Macros

More powerful but require a separate crate:

- **Derive macros**: `#[derive(Debug, Clone)]`
- **Attribute macros**: `#[route(GET, "/")]`
- **Function-like macros**: `sql!(SELECT * FROM users)`

## Run the Examples

```bash
cd day-20
cargo run
```

## Homework

### Homework 1: Getters/Setters Macro
Create a macro that generates getter and setter methods:

```rust
macro_rules! getters_setters {
    ($($name:ident: $type:ty),*) => {
        // Generate get_name and set_name for each field
    }
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    getters_setters!(name: String, age: u32);
}
```

### Homework 2: Enum with Display
Create a macro that defines an enum and implements Display:

```rust
macro_rules! enum_with_display {
    ($name:ident { $($variant:ident),* }) => {
        // Define enum and implement Display
    }
}

enum_with_display!(Color { Red, Green, Blue });
// Now Color implements Display automatically
```

### Homework 3: Retry Macro
Create a macro that retries a fallible operation:

```rust
macro_rules! retry {
    ($times:expr, $block:block) => {
        // Try $block up to $times, return on success
    }
}

let result = retry!(3, {
    potentially_failing_operation()
});
```

---

[← Day 19: Concurrency](../day-19/README.md) | [Home](../README.md) | [Day 21: Final Project →](../day-21/README.md)

# Day 10: Error Handling

## What Will We Learn Today?
- The `Result<T, E>` enum
- Handling errors with `match`
- The `?` operator for error propagation
- `unwrap`, `expect`, and alternatives
- Custom error types

## The Result Enum

Rust uses `Result` for operations that can fail:

```rust
enum Result<T, E> {
    Ok(T),   // Success with value of type T
    Err(E),  // Error with value of type E
}
```

## Handling Errors with match

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

## The ? Operator

Propagates errors automatically:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

## unwrap and expect

Quick ways to get values (panic on error):

```rust
let file = File::open("hello.txt").unwrap();
let file = File::open("hello.txt").expect("Failed to open file");
```

## Safer Alternatives

```rust
// Default value on error
let value = result.unwrap_or(0);

// Compute default on error
let value = result.unwrap_or_else(|e| {
    println!("Error: {}", e);
    0
});

// Convert to Option
let maybe_value = result.ok();
```

## Custom Error Types

```rust
#[derive(Debug)]
enum ValidationError {
    NegativeAge,
    TooOld,
}

fn validate_age(age: i32) -> Result<i32, ValidationError> {
    if age < 0 {
        Err(ValidationError::NegativeAge)
    } else if age > 120 {
        Err(ValidationError::TooOld)
    } else {
        Ok(age)
    }
}
```

## Error Conversion

```rust
fn parse_and_double(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map(|n| n * 2)
        .map_err(|e| format!("Parse error: {}", e))
}
```

## Run the Examples

```bash
cd day-10
cargo run
```

## Homework

### Homework 1: Config Reader
Create a function that reads a configuration file:

```rust
struct Config {
    host: String,
    port: u16,
}

enum ConfigError {
    FileNotFound,
    InvalidFormat,
    MissingField(String),
}

fn read_config(path: &str) -> Result<Config, ConfigError> {
    // Your implementation
}
```

### Homework 2: User Registration with Validation
Create custom errors for user registration:

```rust
enum RegistrationError {
    UsernameTooShort,
    InvalidEmail,
    PasswordTooWeak,
}

fn register_user(
    username: &str, 
    email: &str, 
    password: &str
) -> Result<User, RegistrationError> {
    // Validate and create user
}
```

### Homework 3: Chain of Fallible Operations
Create a processing pipeline with proper error handling:

```rust
enum ProcessError {
    ParseError,
    OutOfRange,
    CalculationError,
}

fn process_input(input: &str) -> Result<i32, ProcessError> {
    // Parse string to number
    // Validate it's in range 1-100
    // Perform calculation
}
```

---

[← Day 9: Enums](../day-09/README.md) | [Home](../README.md) | [Day 11: Collections →](../day-11/README.md)

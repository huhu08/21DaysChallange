// Day 10: Error Handling
// This file contains examples for Day 10 of the 21 Days of Rust Challenge

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("=== Day 10: Error Handling ===\n");

    // Example 1: Result enum
    println!("Example 1: Result<T, E>");
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("Something went wrong");
    println!("Success: {:?}", success);
    println!("Failure: {:?}", failure);
    println!();

    // Example 2: Handling Result with match
    println!("Example 2: Handling Result with match");
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Example 3: unwrap and expect
    println!("Example 3: unwrap and expect");
    let x: Result<i32, &str> = Ok(5);
    println!("unwrap: {}", x.unwrap());

    let y: Result<i32, &str> = Ok(10);
    println!("expect: {}", y.expect("Failed to get value"));
    // Note: unwrap() and expect() will panic on Err
    println!();

    // Example 4: unwrap_or and unwrap_or_else
    println!("Example 4: unwrap_or and unwrap_or_else");
    let result: Result<i32, &str> = Err("error");
    let value = result.unwrap_or(0);
    println!("unwrap_or(0): {}", value);

    let result2: Result<i32, &str> = Err("error");
    let value2 = result2.unwrap_or_else(|e| {
        println!("Handling error: {}", e);
        -1
    });
    println!("unwrap_or_else: {}", value2);
    println!();

    // Example 5: The ? operator
    println!("Example 5: The ? Operator");
    match read_username_from_file() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error reading username: {}", e),
    }
    println!();

    // Example 6: Custom error types
    println!("Example 6: Custom Error Types");
    println!("validate_age(25): {:?}", validate_age(25));
    println!("validate_age(-5): {:?}", validate_age(-5));
    println!("validate_age(150): {:?}", validate_age(150));
    println!();

    // Example 7: Error conversion with map_err
    println!("Example 7: Error Conversion");
    match parse_and_double("42") {
        Ok(n) => println!("Parsed and doubled '42': {}", n),
        Err(e) => println!("Error: {}", e),
    }
    match parse_and_double("not a number") {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }
    println!();

    // Example 8: Option to Result conversion
    println!("Example 8: Option to Result");
    let some_val: Option<i32> = Some(5);
    let result: Result<i32, &str> = some_val.ok_or("No value");
    println!("Some(5).ok_or(): {:?}", result);

    let none_val: Option<i32> = None;
    let result: Result<i32, &str> = none_val.ok_or("No value");
    println!("None.ok_or(): {:?}", result);
    println!();

    // Example 9: Chaining with and_then
    println!("Example 9: Chaining with and_then");
    let result = parse_positive("42").and_then(|n| if n > 10 { Ok(n) } else { Err("Too small") });
    println!("parse_positive('42').and_then(>10): {:?}", result);

    let result = parse_positive("-5");
    println!("parse_positive('-5'): {:?}", result);
    println!();

    println!("=== End of Day 10 Examples ===");
}

// Basic Result example
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Using the ? operator for propagation
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Alternative with longer form
fn read_username_verbose() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Custom error types
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

// Error conversion
fn parse_and_double(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map(|n| n * 2)
        .map_err(|e| format!("Parse error: {}", e))
}

// Chaining operations
fn parse_positive(s: &str) -> Result<i32, &'static str> {
    match s.parse::<i32>() {
        Ok(n) if n > 0 => Ok(n),
        Ok(_) => Err("Number must be positive"),
        Err(_) => Err("Invalid number format"),
    }
}

// Day 7: Week 1 Project - Calculator
// This file contains the Week 1 project for the 21 Days of Rust Challenge
// Build a command-line calculator using everything you've learned!

use std::io;

fn main() {
    println!("=== Day 7: Week 1 Project - Calculator ===\n");
    println!("Welcome to the Rust Calculator!");
    println!(
        "This project combines: Variables, Functions, Control Flow, Ownership, Borrowing, and Slices\n"
    );

    loop {
        println!("\n--- Calculator Menu ---");
        println!("1. Basic Operations (+, -, *, /)");
        println!("2. Power Calculation");
        println!("3. Factorial");
        println!("4. Temperature Converter");
        println!("5. Exit");
        print!("\nChoose an option (1-5): ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();

        match choice {
            "1" => basic_operations(),
            "2" => power_calculation(),
            "3" => factorial_calculation(),
            "4" => temperature_converter(),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn basic_operations() {
    println!("\n--- Basic Operations ---");

    let num1 = get_number("Enter first number: ");
    let num2 = get_number("Enter second number: ");

    println!("\nSelect operation:");
    println!("1. Addition (+)");
    println!("2. Subtraction (-)");
    println!("3. Multiplication (*)");
    println!("4. Division (/)");

    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read line");

    let result = match op.trim() {
        "1" | "+" => Some(add(num1, num2)),
        "2" | "-" => Some(subtract(num1, num2)),
        "3" | "*" => Some(multiply(num1, num2)),
        "4" | "/" => divide(num1, num2),
        _ => {
            println!("Invalid operation");
            None
        }
    };

    if let Some(r) = result {
        println!("Result: {}", r);
    }
}

fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

// Basic arithmetic functions demonstrating ownership and borrowing
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        println!("Error: Cannot divide by zero!");
        None
    } else {
        Some(a / b)
    }
}

fn power_calculation() {
    println!("\n--- Power Calculation ---");

    let base = get_number("Enter base: ");
    let exp = get_number("Enter exponent: ") as u32;

    let result = power(base, exp);
    println!("{} ^ {} = {}", base, exp, result);
}

fn power(base: f64, exp: u32) -> f64 {
    let mut result = 1.0;
    for _ in 0..exp {
        result *= base;
    }
    result
}

fn factorial_calculation() {
    println!("\n--- Factorial Calculation ---");

    let num = get_number("Enter a non-negative integer: ") as u64;

    let result = factorial(num);
    println!("{}! = {}", num, result);
}

fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

fn temperature_converter() {
    println!("\n--- Temperature Converter ---");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => {
            let celsius = get_number("Enter temperature in Celsius: ");
            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("{}°C = {}°F", celsius, fahrenheit);
        }
        "2" => {
            let fahrenheit = get_number("Enter temperature in Fahrenheit: ");
            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("{}°F = {}°C", fahrenheit, celsius);
        }
        _ => println!("Invalid option"),
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

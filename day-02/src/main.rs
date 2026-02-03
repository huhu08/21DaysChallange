// Day 2: Functions
// This file contains examples for Day 2 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 2: Functions ===\n");

    // Example 1: Calling a simple function
    println!("Example 1: Simple Function");
    greet();
    println!();

    // Example 2: Function with parameters
    println!("Example 2: Function with Parameters");
    greet_person("Alice");
    greet_person("Bob");
    println!();

    // Example 3: Function with multiple parameters
    println!("Example 3: Multiple Parameters");
    print_sum(5, 3);
    print_labeled_measurement(10, "meters");
    println!();

    // Example 4: Function with return value
    println!("Example 4: Return Values");
    let result = add(10, 20);
    println!("10 + 20 = {}", result);

    let squared = square(7);
    println!("7 squared = {}", squared);
    println!();

    // Example 5: Expressions vs Statements
    println!("Example 5: Expressions vs Statements");
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {}", y);
    println!();

    // Example 6: Early return
    println!("Example 6: Early Return");
    println!("abs(-5) = {}", absolute_value(-5));
    println!("abs(3) = {}", absolute_value(3));
    println!();

    // Example 7: Nested function calls
    println!("Example 7: Nested Function Calls");
    let result = add(multiply(2, 3), multiply(4, 5));
    println!("(2 * 3) + (4 * 5) = {}", result);
    println!();

    // Example 8: Boolean return
    println!("Example 8: Boolean Return");
    println!("is_even(4) = {}", is_even(4));
    println!("is_even(7) = {}", is_even(7));
    println!();

    // Example 9: Recursive function
    println!("Example 9: Recursive Function");
    println!("factorial(5) = {}", factorial(5));
    println!("fibonacci(10) = {}", fibonacci(10));
    println!();

    println!("=== End of Day 2 Examples ===");
}

fn greet() {
    println!("Hello, World!");
}

fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

fn print_labeled_measurement(value: i32, unit: &str) {
    println!("The measurement is: {} {}", value, unit);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn square(x: i32) -> i32 {
    x * x
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;
    }
    x
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

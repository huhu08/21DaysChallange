// Day 3: Control Flow
// This file contains examples for Day 3 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 3: Control Flow ===\n");

    // Example 1: Basic if-else
    println!("Example 1: Basic if-else");
    let number = 7;
    if number > 5 {
        println!("{} is greater than 5", number);
    } else {
        println!("{} is not greater than 5", number);
    }
    println!();

    // Example 2: if-else if-else chain
    println!("Example 2: if-else if-else Chain");
    let score = 85;
    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }
    println!();

    // Example 3: if as an expression
    println!("Example 3: if as Expression");
    let condition = true;
    let value = if condition { 5 } else { 10 };
    println!("value = {}", value);
    println!();

    // Example 4: loop with break
    println!("Example 4: loop with break");
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            println!("Reached 5, breaking!");
            break;
        }
        println!("Counter: {}", counter);
    }
    println!();

    // Example 5: loop with return value
    println!("Example 5: loop with Return Value");
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("Result from loop: {}", result);
    println!();

    // Example 6: Loop labels
    println!("Example 6: Loop Labels");
    let mut count = 0;
    'outer: loop {
        println!("Outer loop");
        let mut remaining = 10;
        loop {
            println!("  Inner loop, remaining: {}", remaining);
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!();

    // Example 7: while loop
    println!("Example 7: while Loop");
    let mut n = 3;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("LIFTOFF!");
    println!();

    // Example 8: for loop with range
    println!("Example 8: for Loop with Range");
    for i in 1..=5 {
        println!("i = {}", i);
    }
    println!();

    // Example 9: for loop with array
    println!("Example 9: for Loop with Array");
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits {
        println!("I like {}", fruit);
    }
    println!();

    // Example 10: for with enumerate
    println!("Example 10: for with Enumerate");
    let colors = ["red", "green", "blue"];
    for (index, color) in colors.iter().enumerate() {
        println!("Color {}: {}", index, color);
    }
    println!();

    // Example 11: FizzBuzz
    println!("Example 11: FizzBuzz (1-15)");
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
    println!();

    println!("=== End of Day 3 Examples ===");
}

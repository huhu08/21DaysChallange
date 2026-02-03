// Day 4: Ownership
// This file contains examples for Day 4 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 4: Ownership ===\n");

    // Example 1: Stack vs Heap
    println!("Example 1: Stack vs Heap");
    let x = 5;
    let s = String::from("hello");
    println!("Stack value x: {}", x);
    println!("Heap value s: {}", s);
    println!();

    // Example 2: Move semantics
    println!("Example 2: Move Semantics");
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // Error: s1 has been moved
    println!("s2: {}", s2);
    println!();

    // Example 3: Clone (deep copy)
    println!("Example 3: Clone (Deep Copy)");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
    println!();

    // Example 4: Copy trait
    println!("Example 4: Copy Trait");
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    println!();

    // Example 5: Ownership and functions
    println!("Example 5: Ownership and Functions");
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // Error: s has been moved

    let x = 5;
    makes_copy(x);
    println!("x is still valid: {}", x);
    println!();

    // Example 6: Return values and ownership
    println!("Example 6: Return Values and Ownership");
    let s1 = gives_ownership();
    println!("s1: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s3: {}", s3);
    println!();

    // Example 7: Multiple return values
    println!("Example 7: Multiple Return Values");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_tuple(s1);
    println!("The length of '{}' is {}", s2, len);
    println!();

    // Example 8: Scope and drop
    println!("Example 8: Scope and Drop");
    {
        let s = String::from("scoped string");
        println!("Inside scope: {}", s);
    }
    println!("Outside scope - s is dropped");
    println!();

    println!("=== End of Day 4 Examples ===");
}

fn takes_ownership(s: String) {
    println!("takes_ownership: {}", s);
}

fn makes_copy(x: i32) {
    println!("makes_copy: {}", x);
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

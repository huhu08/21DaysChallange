// Day 11: Collections
// This file contains examples for Day 11 of the 21 Days of Rust Challenge

use std::collections::HashMap;

fn main() {
    println!("=== Day 11: Collections ===\n");

    // ============================================================
    // VECTORS
    // ============================================================
    println!("--- Vectors ---\n");

    // Example 1: Creating vectors
    println!("Example 1: Creating Vectors");
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4, 5];
    println!("Empty vector: {:?}", v1);
    println!("Initialized vector: {:?}", v2);
    println!();

    // Example 2: Modifying vectors
    println!("Example 2: Modifying Vectors");
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("After push: {:?}", v);

    let popped = v.pop();
    println!("Popped: {:?}, Vector: {:?}", popped, v);

    v.insert(1, 10); // Insert 10 at index 1
    println!("After insert at 1: {:?}", v);

    v.remove(1); // Remove element at index 1
    println!("After remove at 1: {:?}", v);
    println!();

    // Example 3: Accessing elements
    println!("Example 3: Accessing Elements");
    let v = vec![10, 20, 30, 40, 50];

    // Using index (panics if out of bounds)
    let third = &v[2];
    println!("Index [2]: {}", third);

    // Using get (returns Option)
    match v.get(2) {
        Some(value) => println!("get(2): {}", value),
        None => println!("No element at index 2"),
    }
    println!("get(100): {:?}", v.get(100)); // Safe - returns None
    println!();

    // Example 4: Iterating over vectors
    println!("Example 4: Iterating");
    let v = vec![1, 2, 3, 4, 5];

    print!("Read-only iteration: ");
    for i in &v {
        print!("{} ", i);
    }
    println!();

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i *= 2;
    }
    println!("After mutation: {:?}", v);
    println!();

    // Example 5: Vector with enum (storing different types)
    println!("Example 5: Vector with Enum");
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
    println!();

    // ============================================================
    // STRINGS
    // ============================================================
    println!("--- Strings ---\n");

    // Example 6: Creating strings
    println!("Example 6: Creating Strings");
    let s1 = String::new();
    let s2 = String::from("hello");
    let s3 = "world".to_string();
    println!("Empty: '{}', From: '{}', to_string: '{}'", s1, s2, s3);
    println!();

    // Example 7: Modifying strings
    println!("Example 7: Modifying Strings");
    let mut s = String::from("foo");
    s.push_str("bar"); // Append string slice
    s.push('!'); // Append single char
    println!("After push_str and push: {}", s);

    // Concatenation with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // Note: s1 is moved
    println!("Concatenated: {}", s3);

    // format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Formatted: {}", s);
    println!();

    // Example 8: String iteration
    println!("Example 8: String Iteration");
    let hello = "Привет"; // Russian "Hello"
    println!("Bytes:");
    for b in hello.bytes() {
        print!("{} ", b);
    }
    println!();
    println!("Chars:");
    for c in hello.chars() {
        print!("{} ", c);
    }
    println!("\n");

    // ============================================================
    // HASHMAPS
    // ============================================================
    println!("--- HashMaps ---\n");

    // Example 9: Creating HashMaps
    println!("Example 9: Creating HashMaps");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:?}", scores);
    println!();

    // Example 10: Accessing values
    println!("Example 10: Accessing Values");
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue team score: {:?}", score);

    // Iterating
    println!("All scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
    println!();

    // Example 11: Updating HashMaps
    println!("Example 11: Updating HashMaps");

    // Overwriting
    scores.insert(String::from("Blue"), 25);
    println!("After overwrite Blue: {:?}", scores);

    // Only insert if key doesn't exist
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Won't change
    println!("After entry().or_insert(): {:?}", scores);
    println!();

    // Example 12: Word count
    println!("Example 12: Word Count");
    let text = "hello world wonderful world hello rust";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Text: '{}'", text);
    println!("Word count: {:?}", word_count);
    println!();

    println!("=== End of Day 11 Examples ===");
}

// Enum for storing different types in a vector
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

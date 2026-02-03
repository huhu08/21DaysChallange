// Day 1: Variables and Data Types
// This file contains examples for Day 1 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 1: Variables and Data Types ===\n");

    // Example 1: Immutable variables (default)
    println!("Example 1: Immutable Variables");
    let x = 5;
    println!("x = {}", x);
    // x = 6; // This would cause an error - variables are immutable by default
    println!();

    // Example 2: Mutable variables
    println!("Example 2: Mutable Variables");
    let mut y = 10;
    println!("y = {}", y);
    y = 20;
    println!("y after mutation = {}", y);
    println!();

    // Example 3: Constants
    println!("Example 3: Constants");
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);
    println!();

    // Example 4: Shadowing
    println!("Example 4: Shadowing");
    let z = 5;
    println!("z = {}", z);
    let z = z + 1;
    println!("z after shadowing = {}", z);
    let z = "now I'm a string!";
    println!("z as string = {}", z);
    println!();

    // Example 5: Integer types
    println!("Example 5: Integer Types");
    let a: i8 = -128;
    let b: u8 = 255;
    let c: i32 = -2_147_483_648;
    let d: u64 = 18_446_744_073_709_551_615;
    println!("i8: {}, u8: {}, i32: {}, u64: {}", a, b, c, d);
    println!();

    // Example 6: Floating point types
    println!("Example 6: Floating Point Types");
    let f1 = 2.5;
    let f2: f32 = 3.14;
    println!("f64: {}, f32: {}", f1, f2);
    println!();

    // Example 7: Boolean type
    println!("Example 7: Boolean Type");
    let is_active: bool = true;
    let is_greater = 10 > 5;
    println!("is_active: {}, is_greater: {}", is_active, is_greater);
    println!();

    // Example 8: Character type
    println!("Example 8: Character Type");
    let letter = 'A';
    let emoji = '🦀';
    let chinese = '中';
    println!("letter: {}, emoji: {}, chinese: {}", letter, emoji, chinese);
    println!();

    // Example 9: Tuples
    println!("Example 9: Tuples");
    let tup: (i32, f64, char) = (500, 6.4, 'R');
    let (x, y, z) = tup;
    println!("Tuple: ({}, {}, {})", x, y, z);
    println!(
        "Access by index: tup.0 = {}, tup.1 = {}, tup.2 = {}",
        tup.0, tup.1, tup.2
    );
    println!();

    // Example 10: Arrays
    println!("Example 10: Arrays");
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("Array: {:?}", arr);
    println!("First element: {}, Second element: {}", first, second);

    let zeros = [0; 5];
    println!("Array of zeros: {:?}", zeros);
    println!();

    println!("=== End of Day 1 Examples ===");
}

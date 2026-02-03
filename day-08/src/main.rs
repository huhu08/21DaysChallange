// Day 8: Structs
// This file contains examples for Day 8 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 8: Structs ===\n");

    // Example 1: Basic struct
    println!("Example 1: Basic Struct");
    let user1 = User {
        username: String::from("alice123"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("User: {}, Email: {}", user1.username, user1.email);
    println!();

    // Example 2: Mutable struct
    println!("Example 2: Mutable Struct");
    let mut user2 = User {
        username: String::from("bob456"),
        email: String::from("bob@example.com"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("bob.new@example.com");
    println!("Updated email: {}", user2.email);
    println!();

    // Example 3: Struct update syntax
    println!("Example 3: Struct Update Syntax");
    let user3 = User {
        email: String::from("charlie@example.com"),
        username: String::from("charlie789"),
        ..user1
    };
    println!("User3: {}, Active: {}", user3.username, user3.active);
    println!();

    // Example 4: Tuple structs
    println!("Example 4: Tuple Structs");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);
    println!();

    // Example 5: Unit-like structs
    println!("Example 5: Unit-like Struct");
    let _always_equal = AlwaysEqual;
    println!("Unit-like struct created (no fields)");
    println!();

    // Example 6: Methods
    println!("Example 6: Struct with Methods");
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!();

    // Example 7: Methods with parameters
    println!("Example 7: Methods with Parameters");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    println!();

    // Example 8: Associated functions
    println!("Example 8: Associated Functions");
    let square = Rectangle::square(25);
    println!("Square: {:?}, Area: {}", square, square.area());
    println!();

    // Example 9: Multiple impl blocks
    println!("Example 9: Multiple impl Blocks");
    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    println!("Is square? {}", rect.is_square());
    let scaled = rect.scale(2);
    println!("Scaled: {:?}", scaled);
    println!();

    println!("=== End of Day 8 Examples ===");
}

// Basic struct definition
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

// Struct with Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// First impl block
impl Rectangle {
    // Method: takes &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // Method with another Rectangle parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function (no self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Second impl block (multiple blocks allowed)
impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn scale(&self, factor: u32) -> Rectangle {
        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}

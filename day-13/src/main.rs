// Day 13: Traits
// This file contains examples for Day 13 of the 21 Days of Rust Challenge

use std::fmt::{self, Debug, Display};

fn main() {
    println!("=== Day 13: Traits ===\n");

    // Example 1: Basic trait
    println!("Example 1: Basic Trait Implementation");
    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released!"),
        author: String::from("Jane Doe"),
        content: String::from("The Rust team announced..."),
    };
    println!("Article summary: {}", article.summarize());

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Exciting news about Rust!"),
        retweets: 100,
    };
    println!("Tweet summary: {}", tweet.summarize());
    println!();

    // Example 2: Default implementation
    println!("Example 2: Default Implementation");
    println!("Article default: {}", article.default_summary());
    println!("Tweet default: {}", tweet.default_summary());
    println!();

    // Example 3: Traits as parameters
    println!("Example 3: Traits as Parameters");
    notify(&article);
    notify(&tweet);
    println!();

    // Example 4: Trait bounds syntax
    println!("Example 4: Trait Bounds");
    notify_with_bounds(&article);
    println!();

    // Example 5: Derivable traits
    println!("Example 5: Derivable Traits");
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = p1.clone();
    println!("p1: {:?}", p1);
    println!("p1 == p2: {}", p1 == p2);
    println!("p3 (cloned): {:?}", p3);
    println!();

    // Example 6: Custom Display trait
    println!("Example 6: Implementing Display");
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Person: {}", person);
    println!("Debug: {:?}", person);
    println!();

    // Example 7: Multiple trait bounds
    println!("Example 7: Multiple Trait Bounds");
    let item = DisplayDebugItem { value: 42 };
    print_item(&item);
    println!();

    // Example 8: Trait objects (dynamic dispatch)
    println!("Example 8: Trait Objects");
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 4.0,
            height: 6.0,
        }),
        Box::new(Triangle {
            base: 3.0,
            height: 4.0,
        }),
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape {}: area = {:.2}", i + 1, shape.area());
    }
    println!();

    // Example 9: Returning traits with impl Trait
    println!("Example 9: Returning impl Trait");
    let summarizable = create_tweet();
    println!("Created: {}", summarizable.summarize());
    println!();

    // Example 10: Supertraits
    println!("Example 10: Supertraits");
    let outlined = OutlinedCircle { radius: 3.0 };
    print_outlined(&outlined);
    println!();

    println!("=== End of Day 13 Examples ===");
}

// Define a trait
trait Summary {
    // Required method
    fn summarize(&self) -> String;

    // Default implementation
    fn default_summary(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    // Method with default that can be overridden
    fn summarize_author(&self) -> String {
        String::from("Unknown")
    }
}

// Implementing trait for NewsArticle
struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

// Implementing trait for Tweet
struct Tweet {
    username: String,
    content: String,
    retweets: u32,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "@{}: {} ({} retweets)",
            self.username, self.content, self.retweets
        )
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Trait as parameter (impl Trait syntax)
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax (equivalent to above)
fn notify_with_bounds<T: Summary>(item: &T) {
    println!("Notification: {}", item.summarize());
}

// Derivable traits
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Implementing Display trait
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

// Multiple trait bounds
struct DisplayDebugItem {
    value: i32,
}

impl Display for DisplayDebugItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value: {}", self.value)
    }
}

impl Debug for DisplayDebugItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DisplayDebugItem {{ value: {} }}", self.value)
    }
}

fn print_item<T: Display + Debug>(item: &T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

// Trait objects for dynamic dispatch
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// Returning impl Trait
fn create_tweet() -> impl Summary {
    Tweet {
        username: String::from("rust_daily"),
        content: String::from("Tip of the day: Use traits!"),
        retweets: 50,
    }
}

// Supertraits - OutlinePrint requires Display
trait OutlinePrint: Display {
    fn outline_print(&self) {
        println!("* {} *", self);
    }
}

struct OutlinedCircle {
    radius: f64,
}

impl Display for OutlinedCircle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle(r={})", self.radius)
    }
}

impl OutlinePrint for OutlinedCircle {}

fn print_outlined(item: &impl OutlinePrint) {
    item.outline_print();
}

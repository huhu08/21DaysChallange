// Day 12: Generics
// This file contains examples for Day 12 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 12: Generics ===\n");

    // Example 1: Generic function
    println!("Example 1: Generic Function");
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Numbers: {:?}", numbers);
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Chars: {:?}", chars);
    println!("Largest char: {}", largest(&chars));
    println!();

    // Example 2: Generic struct with same type
    println!("Example 2: Generic Struct (Same Type)");
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 4.2 };
    println!("Integer point: ({}, {})", int_point.x, int_point.y);
    println!("Float point: ({}, {})", float_point.x, float_point.y);
    println!();

    // Example 3: Generic struct with multiple types
    println!("Example 3: Multiple Generic Types");
    let mixed = MixedPoint { x: 5, y: 4.0 };
    println!("Mixed point: ({}, {})", mixed.x, mixed.y);

    let string_int = MixedPoint { x: "Hello", y: 42 };
    println!("String-Int point: ({}, {})", string_int.x, string_int.y);
    println!();

    // Example 4: Generic methods
    println!("Example 4: Generic Methods");
    let p = Point { x: 5, y: 10 };
    println!("p.x() = {}", p.x());
    println!("p.y() = {}", p.y());
    println!();

    // Example 5: Type-specific methods
    println!("Example 5: Type-Specific Methods");
    let float_point = Point {
        x: 3.0_f32,
        y: 4.0_f32,
    };
    println!("Point: ({}, {})", float_point.x, float_point.y);
    println!(
        "Distance from origin: {}",
        float_point.distance_from_origin()
    );
    // Note: int_point.distance_from_origin() would not compile
    println!();

    // Example 6: Mixing generic types in methods
    println!("Example 6: Mixing Types in Methods");
    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3: ({}, {})", p3.x, p3.y);
    println!();

    // Example 7: Generic enum
    println!("Example 7: Generic Enums");
    let some_number: MyOption<i32> = MyOption::Some(5);
    let some_string: MyOption<String> = MyOption::Some(String::from("hello"));
    let none: MyOption<i32> = MyOption::None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("none: {:?}", none);
    println!();

    // Example 8: Generic Result-like enum
    println!("Example 8: Result-like Generic Enum");
    let success: MyResult<i32, String> = MyResult::Ok(42);
    let failure: MyResult<i32, String> = MyResult::Err(String::from("error"));
    println!("success: {:?}", success);
    println!("failure: {:?}", failure);
    println!();

    // Example 9: Generic data structure - Stack
    println!("Example 9: Generic Stack");
    let mut int_stack: Stack<i32> = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);
    println!("Stack: {:?}", int_stack);
    println!("Pop: {:?}", int_stack.pop());
    println!("Peek: {:?}", int_stack.peek());
    println!("Is empty: {}", int_stack.is_empty());
    println!();

    // Example 10: Generic with trait bounds
    println!("Example 10: Generic with Trait Bounds");
    let pair = Pair::new(10, 20);
    pair.cmp_display();
    println!();

    println!("=== End of Day 12 Examples ===");
}

// Generic function - finds largest element
// T must implement PartialOrd for comparison
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct with single type parameter
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Methods for any Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// Methods only for Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic struct with multiple type parameters
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    // Method that mixes types from two different points
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// Generic enum similar to Option<T>
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

// Generic enum with two type parameters (like Result<T, E>)
#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

// Generic data structure
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// Generic with trait bounds
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

// This method only exists for Pair<T> where T implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

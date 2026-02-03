// Day 16: Closures
// This file contains examples for Day 16 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 16: Closures ===\n");

    // Example 1: Basic closure syntax
    println!("Example 1: Basic Closure Syntax");

    // Different ways to define closures
    let add_one_v1 = |x: i32| -> i32 { x + 1 }; // Fully annotated
    let add_one_v2 = |x: i32| x + 1; // Without return type
    let add_one_v3 = |x: i32| x + 1; // Without braces
    let add_one_v4 = |x| x + 1; // Type inference

    println!("add_one_v1(5) = {}", add_one_v1(5));
    println!("add_one_v2(5) = {}", add_one_v2(5));
    println!("add_one_v3(5) = {}", add_one_v3(5));
    println!("add_one_v4(5) = {}", add_one_v4(5));
    println!();

    // Example 2: Capturing environment - immutable borrow
    println!("Example 2: Capturing by Immutable Reference");
    let x = 4;
    let equal_to_x = |z| z == x; // Captures x by reference
    println!("x = {}", x);
    println!("Is 4 equal to x? {}", equal_to_x(4));
    println!("Is 5 equal to x? {}", equal_to_x(5));
    println!("x is still accessible: {}", x);
    println!();

    // Example 3: Capturing environment - mutable borrow
    println!("Example 3: Capturing by Mutable Reference");
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count inside closure: {}", count);
    };
    increment();
    increment();
    increment();
    println!("Final count: {}", count);
    println!();

    // Example 4: Move closure - taking ownership
    println!("Example 4: Move Closure");
    let data = vec![1, 2, 3];
    let print_data = move || {
        println!("Data in closure: {:?}", data);
    };
    print_data();
    print_data();
    // data is no longer accessible here - it was moved into the closure
    // println!("{:?}", data);  // This would cause a compile error
    println!();

    // Example 5: Closures with iterators
    println!("Example 5: Closures with Iterators");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).cloned().collect();
    println!("Evens: {:?}", evens);

    let sum: i32 = numbers.iter().filter(|x| *x % 2 == 0).sum();
    println!("Sum of evens: {}", sum);
    println!();

    // Example 6: Closures as function parameters
    println!("Example 6: Closures as Parameters");

    let square = |x| x * x;
    let double = |x| x * 2;

    apply_and_print(5, square);
    apply_and_print(5, double);
    apply_and_print(5, |x| x + 10); // Inline closure
    println!();

    // Example 7: Different closure traits
    println!("Example 7: Fn, FnMut, FnOnce");

    // FnOnce - can be called once, may consume captured values
    let consume_vec = || {
        let v = vec![1, 2, 3];
        v // Returns ownership
    };
    let result = consume_vec();
    println!("Consumed: {:?}", result);

    // FnMut - can be called multiple times, may mutate captured values
    let mut counter = Counter { value: 0 };
    let mut increment = || counter.increment();
    increment();
    increment();
    println!("Counter value: {}", counter.value);

    // Fn - can be called multiple times, only reads captured values
    let x = 10;
    let read_x = || x * 2;
    println!("read_x(): {}", read_x());
    println!("read_x(): {}", read_x());
    println!();

    // Example 8: Returning closures
    println!("Example 8: Returning Closures");
    let add_five = make_adder(5);
    let add_ten = make_adder(10);
    println!("add_five(3) = {}", add_five(3));
    println!("add_ten(3) = {}", add_ten(3));
    println!();

    // Example 9: Storing closures in structs
    println!("Example 9: Closures in Structs");
    let mut cache = Cacher::new(|x| {
        println!("Computing expensive value...");
        x * 2
    });
    println!("First call: {}", cache.value(5));
    println!("Second call (cached): {}", cache.value(5));
    println!();

    // Example 10: Practical example - configuration with callbacks
    println!("Example 10: Practical - Event Handlers");
    let mut button = Button {
        label: String::from("Click me"),
        on_click: None,
    };

    button.set_on_click(|| println!("Button was clicked!"));
    button.click();
    button.click();
    println!();

    println!("=== End of Day 16 Examples ===");
}

// Function that takes a closure as parameter
fn apply_and_print<F>(x: i32, f: F)
where
    F: Fn(i32) -> i32,
{
    println!("f({}) = {}", x, f(x));
}

// Function that returns a closure
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// Helper struct for FnMut example
struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}

// Struct that stores a closure (memoization pattern)
struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    calculation: T,
    value: Option<i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// Button with optional callback
struct Button {
    label: String,
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    fn set_on_click<F>(&mut self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.on_click = Some(Box::new(callback));
    }

    fn click(&self) {
        println!("Button '{}' pressed", self.label);
        if let Some(ref callback) = self.on_click {
            callback();
        }
    }
}

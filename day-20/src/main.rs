// Day 20: Macros - cleaned version

// Macro definitions (simple demonstrations)
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}
macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

macro_rules! calculate_add {
    ($a:expr, $b:expr) => {
        println!("{} + {} = {}", $a, $b, $a + $b);
    };
}
macro_rules! calculate_sub {
    ($a:expr, $b:expr) => {
        println!("{} - {} = {}", $a, $b, $a - $b);
    };
}
macro_rules! calculate_mul {
    ($a:expr, $b:expr) => {
        println!("{} * {} = {}", $a, $b, $a * $b);
    };
}

macro_rules! create_vec { ( $( $x:expr ),* ) => { { let mut v = Vec::new(); $( v.push($x); )* v } }; }
macro_rules! print_all { ( $( $x:expr ),* ) => { $( println!("  Value: {:?}", $x); )* }; }
macro_rules! double_vec { ( $( $x:expr ),* ) => { { let mut v = Vec::new(); $( v.push($x * 2); )* v; } }; }
struct Person {
    name: String,
    age: u32,
}
macro_rules! create_person {
    ($var:ident, $name:expr, $age:expr) => {
        let $var = Person {
            name: String::from($name),
            age: $age,
        };
    };
}
trait Describable {
    fn describe(&self) -> String;
}
macro_rules! impl_describable {
    ($type:ty, $format:expr) => {
        impl Describable for $type {
            fn describe(&self) -> String {
                format!($format, self.x, self.y)
            }
        }
    };
}
struct Point {
    x: i32,
    y: i32,
}
struct Circle {
    radius: f64,
}
impl_describable!(Point, "Point at ({}, {})");
impl Describable for Circle {
    fn describe(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
macro_rules! debug_print {
    ($msg:expr) => {
        #[cfg(debug_assertions)]
        println!("  DEBUG: {}", $msg);
    };
}
macro_rules! hashmap { ( $( $key:expr => $value:expr ),* ) => { { let mut map = std::collections::HashMap::new(); $( map.insert($key, $value); )* map } }; }
fn measure_time<F: FnOnce()>(name: &str, f: F) {
    let start = std::time::Instant::now();
    f();
    let duration = start.elapsed();
    println!("  {} took {:?}", name, duration);
}
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    println!("=== Day 20: Macros ===\n");

    println!("Example 1: Built-in Macros");
    println!("println! is a macro");
    let v = vec![1, 2, 3];
    println!("vec! macro: {:?}", v);
    println!("format! macro: {}", format!("Hello, {}!", "Rust"));
    println!();

    println!("Example 2: Simple Declarative Macro");
    say_hello!();
    println!();

    println!("Example 3: Macro with Arguments");
    greet!("Alice");
    greet!("Bob");
    println!();

    println!("Example 4: Multiple Patterns");
    calculate_add!(5, 3);
    calculate_sub!(10, 4);
    calculate_mul!(6, 7);
    println!();

    println!("Example 5: Repetition");
    let v2 = create_vec![1, 2, 3, 4, 5];
    println!("create_vec!: {:?}", v2);
    print_all!(1, "hello", 3.14, true);
    println!();

    println!("Example 6: Custom Vec Macro");
    let doubled = double_vec![1, 2, 3, 4, 5];
    println!("double_vec!: {:?}", doubled);
    println!();

    println!("Example 7: Struct Creation Macro");
    create_person!(alice, "Alice", 30);
    create_person!(bob, "Bob", 25);
    println!("alice: {}, age {}", alice.name, alice.age);
    println!("bob: {}, age {}", bob.name, bob.age);
    println!();

    println!("Example 8: Trait Implementation Macro");
    let point = Point { x: 3, y: 4 };
    println!("Point: {}", point.describe());
    let circle = Circle { radius: 5.0 };
    println!("Circle: {}", circle.describe());
    println!();

    println!("Example 9: Conditional Compilation");
    #[cfg(debug_assertions)]
    println!("  Debug mode is enabled");
    #[cfg(not(debug_assertions))]
    println!("  Release mode is enabled");
    debug_print!("This only prints in debug mode");
    println!();

    println!("Example 10: Useful Patterns");
    let scores = hashmap! {
        "Alice" => 100,
        "Bob" => 85,
        "Charlie" => 92
    };
    println!("Scores: {:?}", scores);
    measure_time("sleep operation", || {
        std::thread::sleep(std::time::Duration::from_millis(10));
    });
    println!();

    println!("Example 11: Macro Hygiene");
    let x = 5;
    let result = square!(x);
    println!("square!({}) = {}", x, result);
    println!();

    println!("Example 12: Procedural Macros (Overview)");
    println!("Procedural macros are more powerful:");
    println!("  - #[derive(...)] - derive macros");
    println!("  - #[attribute] - attribute macros");
    println!("  - function_like!() - function-like macros");
    println!("They require a separate crate to implement.");
    println!("\n=== End of Day 20 Examples ===");
}

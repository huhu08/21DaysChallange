// Day 6: Week 1 Project - Simple Calculator
// This file contains a mini project combining all Week 1 concepts

fn main() {
    println!("=== Day 6: Week 1 Project - Calculator ===\n");

    println!("Welcome to the Rust Calculator!");
    println!("This calculator demonstrates Week 1 concepts.\n");

    // Basic operations demo
    demo_basic_operations();
    demo_advanced_operations();

    // Calculator with Option
    println!("\n--- Safe Calculations ---\n");

    let result = calculate(10.0, 5.0, '+');
    println!("10 + 5 = {:?}", result);

    let result = calculate(10.0, 0.0, '/');
    println!("10 / 0 = {:?}", result);

    // Calculator struct demo
    println!("\n--- Calculator with History ---\n");
    let mut calc = Calculator::new();

    calc.add(10.0);
    calc.subtract(3.0);
    calc.multiply(2.0);
    calc.divide(7.0);

    calc.print_history();
    println!("Final result: {}", calc.get_result());

    println!("\n=== End of Week 1 Project ===");
}

fn demo_basic_operations() {
    println!("--- Basic Operations ---");
    let a = 15.0;
    let b = 4.0;

    println!("{} + {} = {}", a, b, add(a, b));
    println!("{} - {} = {}", a, b, subtract(a, b));
    println!("{} * {} = {}", a, b, multiply(a, b));
    println!("{} / {} = {}", a, b, divide(a, b));
}

fn demo_advanced_operations() {
    println!("\n--- Advanced Operations ---");
    println!("Square root of 16 = {}", square_root(16.0));
    println!("2^10 = {}", power(2.0, 10.0));
    println!("Factorial of 5 = {}", factorial(5));
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 { f64::NAN } else { a / b }
}
fn square_root(n: f64) -> f64 {
    n.sqrt()
}
fn power(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

fn calculate(a: f64, b: f64, op: char) -> Option<f64> {
    match op {
        '+' => Some(a + b),
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' => {
            if b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
        _ => None,
    }
}

struct Calculator {
    result: f64,
    history: Vec<String>,
}

impl Calculator {
    fn new() -> Calculator {
        Calculator {
            result: 0.0,
            history: Vec::new(),
        }
    }

    fn add(&mut self, value: f64) {
        let old = self.result;
        self.result += value;
        self.history
            .push(format!("{} + {} = {}", old, value, self.result));
    }

    fn subtract(&mut self, value: f64) {
        let old = self.result;
        self.result -= value;
        self.history
            .push(format!("{} - {} = {}", old, value, self.result));
    }

    fn multiply(&mut self, value: f64) {
        let old = self.result;
        self.result *= value;
        self.history
            .push(format!("{} * {} = {}", old, value, self.result));
    }

    fn divide(&mut self, value: f64) {
        if value != 0.0 {
            let old = self.result;
            self.result /= value;
            self.history
                .push(format!("{} / {} = {}", old, value, self.result));
        }
    }

    fn get_result(&self) -> f64 {
        self.result
    }

    fn print_history(&self) {
        println!("Calculation History:");
        for (i, entry) in self.history.iter().enumerate() {
            println!("  {}. {}", i + 1, entry);
        }
    }
}

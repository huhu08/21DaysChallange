// Day 0: Introduction to Rust
// This file contains examples for Day 0 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 0: Introduction to Rust ===\n");

    // Example 1: Hello World
    println!("Example 1: Hello World");
    println!("Hello, World!");
    println!();

    // Example 2: Multiple print statements
    println!("Example 2: Multiple Print Statements");
    println!("Line 1: Welcome to Rust!");
    println!("Line 2: Rust is a systems programming language.");
    println!("Line 3: It focuses on safety and performance.");
    println!();

    // Example 4: Using print! (without newline)
    println!("Example 4: Using print! (without newline)");
    print!("This ");
    print!("is ");
    print!("on ");
    print!("one ");
    print!("line!");
    println!();
    println!();

    // Example 5: Escape characters
    println!("Example 5: Escape Characters");
    println!("Tab character:\tHello");
    println!("New line in string:\nThis is a new line");
    println!("Backslash: \\");
    println!("Double quote: \"Hello\"");
    println!();

    // Example 6: Raw strings
    println!("Example 6: Raw Strings");
    println!(r"This is a raw string: \n is not a newline here");
    println!(r#"You can use "quotes" in raw strings"#);
    println!();

    // Example 7: Why Rust?
    println!("Example 7: Why Learn Rust?");
    print_why_rust();
    println!();

    println!("=== End of Day 0 Examples===\n");

    // Goodbye message
    print_21_days_banner();
}

/// 21 Days Challenge Banner
fn print_21_days_banner() {
    println!(
        r#"
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║   Day 0 Complete! See you tomorrow for Variables!         ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
    "#
    );
}

/// Why Rust - Key Features
fn print_why_rust() {
    println!(
        r#"
    ┌─────────────────────────────────────────────┐
    │              WHY RUST? 🦀                   │
    ├─────────────────────────────────────────────┤
    │                                             │
    │  ⚡ PERFORMANCE                             │
    │     Zero-cost abstractions                  │
    │     No garbage collector                    │
    │     Minimal runtime                         │
    │                                             │
    │  🛡️  RELIABILITY                            │
    │     Memory safety guaranteed                │
    │     No null pointer exceptions              │
    │     No data races                           │
    │                                             │
    │  🔧 PRODUCTIVITY                            │
    │     Great documentation                     │
    │     Helpful compiler messages               │
    │     Cargo package manager                   │
    │                                             │
    └─────────────────────────────────────────────┘
    "#
    );
}

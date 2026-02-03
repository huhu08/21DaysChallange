# Day 0: Introduction to Rust

```
    🦀 Welcome to 21 Days of Rust! 🦀

              _~^~^~_
          \) /  o o  \ (/
            '_   ¬   _'
            / '-----' \
           /     |     \
          /      |      \
         /_______|_______\

         Hello, I'm Ferris!
       Your Rust learning buddy
```

## 🇹🇷 Türkiye Rust Community

```
    ╔═══════════════════════════════════════════╗
    ║                                           ║
    ║      🇹🇷  TÜRKİYE RUST COMMUNITY  🇹🇷      ║
    ║                                           ║
    ║        Güvenli • Hızlı • Eşzamanlı        ║
    ║        Safe   • Fast  • Concurrent        ║
    ║                                           ║
    ╚═══════════════════════════════════════════╝
```

## What Will We Learn Today?
- What is Rust and why should we use it?
- Rust installation
- Cargo: Rust's package manager
- Our first Rust program: "Hello, World!"

## What is Rust?

Rust is a systems programming language focused on **safety**, **speed**, and **concurrency**.

```
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
```

### Key Advantages:
- **Memory Safety**: No garbage collector needed
- **Zero-Cost Abstractions**: High-level features with low-level performance
- **Fearless Concurrency**: Data races caught at compile time
- **Modern Tooling**: Cargo, rustfmt, clippy, rust-analyzer

## Installation

### macOS / Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows
Download and run [rustup-init.exe](https://rustup.rs)

### Verify Installation
```bash
rustc --version
# rustc 1.XX.X (xxxxx 202X-XX-XX)

cargo --version
# cargo 1.XX.X (xxxxx 202X-XX-XX)
```

## Hello, World!

Create a new project:
```bash
cargo new hello_rust
cd hello_rust
```

Edit `src/main.rs`:
```rust
fn main() {
    println!("Hello, World!");
}
```

Run:
```bash
cargo run
```

## Basic Cargo Commands

| Command | Description |
|---------|-------------|
| `cargo new <name>` | Create a new project |
| `cargo build` | Compile the project |
| `cargo run` | Compile and run |
| `cargo check` | Quick compilation check (no binary) |
| `cargo test` | Run tests |
| `cargo doc` | Generate documentation |
| `cargo fmt` | Format code |
| `cargo clippy` | Lint code |

## Project Structure

```
my_project/
├── Cargo.toml       # Project configuration
├── Cargo.lock       # Dependency lock file
└── src/
    └── main.rs      # Entry point
```

### Cargo.toml
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
```

## Run the Examples

```bash
cd day-00
cargo run
```

You'll see Ferris the Crab welcome you to the challenge!

## Homework

### Homework 1: Create Your Own Greeting
Create a function that prints a personalized greeting message.

```rust
fn my_greeting() {
    println!("Hello from [Your Name]!");
    println!("I'm starting my Rust journey today!");
}
```

### Homework 2: ASCII Art
Create your own ASCII art function. Get creative!

```rust
fn my_ascii_art() {
    println!(r#"
       /\_/\
      ( o.o )
       > ^ <
      /|   |\
     (_|   |_)
    "#);
}
```

### Homework 3: Experiment with Print Macros
Try these variations:
- `print!` - without newline
- `println!` - with newline
- `eprint!` - to stderr
- `eprintln!` - to stderr with newline

### Homework 4: Escape Characters
Experiment with escape sequences:
- `\n` - newline
- `\t` - tab
- `\\` - backslash
- `\"` - double quote
- `\r` - carriage return

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn through examples
- [Rust Playground](https://play.rust-lang.org/) - Try Rust in browser
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises

---

```
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║   Day 0 Complete! 🎉                                      ║
    ║                                                           ║
    ║   Tomorrow: Variables & Mutability                        ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
```

[Home](../README.md) | [Day 1: Variables →](../day-01/README.md)

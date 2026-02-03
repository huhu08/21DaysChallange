# 21 Days of Rust - Learn from Scratch Challenge

A comprehensive curriculum designed to learn Rust programming language from scratch in 21 days (Day 0 to Day 21).

## Table of Contents

- [About](#about)
- [How to Use](#how-to-use)
- [Curriculum](#curriculum)
- [Requirements](#requirements)
- [Contributing](#contributing)

## About

This repository is a 21-day learning journey designed for complete beginners to the Rust programming language. Each day includes theory, code examples, and homework exercises.

**What makes Rust special?**
- Memory safety without garbage collection
- Zero-cost abstractions
- Fearless concurrency
- Expressive type system

## How to Use

1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/21DaysChallange.git
   cd 21DaysChallange
   ```

2. Navigate to the day folder and read the README:
   ```bash
   cd day-00
   cat README.md
   ```

3. Run the examples:
   ```bash
   cargo run
   ```

4. Complete the homework exercises in per-day README.md files.

## Curriculum

### Week 0: Getting Started
| Day | Topic | Description |
|-----|-------|-------------|
| 0 | [Introduction to Rust](./day-00/README.md) | Installation, Cargo, Hello World |

### Week 1: Fundamentals (Day 1-7)
| Day | Topic | Description |
|-----|-------|-------------|
| 1 | [Variables & Mutability](./day-01/README.md) | let, mut, const, shadowing |
| 2 | [Data Types](./day-02/README.md) | Scalars, compounds, type inference |
| 3 | [Functions](./day-03/README.md) | Parameters, return values, expressions |
| 4 | [Control Flow](./day-04/README.md) | if/else, loops, match |
| 5 | [Ownership](./day-05/README.md) | Move semantics, Copy trait |
| 6 | [Borrowing & Slices](./day-06/README.md) | References, string/array slices |
| 7 | [**Week 1 Project**](./day-07/README.md) | **Calculator** |

### Week 2: Core Concepts (Day 8-14)
| Day | Topic | Description |
|-----|-------|-------------|
| 8 | [Structs](./day-08/README.md) | Fields, methods, associated functions |
| 9 | [Enums & Pattern Matching](./day-09/README.md) | Variants, match, Option |
| 10 | [Error Handling](./day-10/README.md) | Result, ?, custom errors |
| 11 | [Collections](./day-11/README.md) | Vec, String, HashMap |
| 12 | [Generics](./day-12/README.md) | Type parameters, bounds |
| 13 | [Traits](./day-13/README.md) | Shared behavior, impl, dyn |
| 14 | [**Week 2 Project**](./day-14/README.md) | **Task Manager** |

### Week 3: Advanced Topics (Day 15-21)
| Day | Topic | Description |
|-----|-------|-------------|
| 15 | [Lifetimes](./day-15/README.md) | Lifetime annotations, elision |
| 16 | [Closures](./day-16/README.md) | Anonymous functions, Fn traits |
| 17 | [Iterators](./day-17/README.md) | Iterator pattern, adaptors |
| 18 | [Smart Pointers](./day-18/README.md) | Box, Rc, RefCell, Weak |
| 19 | [Concurrency](./day-19/README.md) | Threads, channels, Mutex, Arc |
| 20 | [Macros](./day-20/README.md) | Declarative macros (macro_rules!) |
| 21 | [**Final Project**](./day-21/README.md) | **Todo CLI Application** |

## Notes and Updates

- Homework is documented in the per-day README.md files.
- Day 20: Macros content has been cleaned up and is described in day-20/README.md.
- Day 21: Final Project tasks and guidance are in day-21/README.md.
- Automation: A script at `scripts/check_homework_sync.sh` verifies that each day README contains a Homework section. Run:
  - `chmod +x scripts/check_homework_sync.sh`
  - `./scripts/check_homework_sync.sh`
- You can run each day independently by following the per-day READMEs (e.g., cd day-00 && cargo run).

## Requirements

- [Rust](https://rustup.rs/) (installation via rustup is recommended)
- A code editor (VS Code + rust-analyzer recommended)
- Basic terminal/command line knowledge

### Rust Installation

```bash
# Unix/Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download and run rustup-init.exe from https://rustup.rs

# Verify installation
rustc --version
cargo --version
```

## Project Structure

```
21DaysChallange/
├── README.md
├── day-00/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
├── day-01/
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── main.rs
...
└── day-21/
    ├── Cargo.toml
    ├── README.md
    └── src/
        └── main.rs
```

Each day folder contains:
- `Cargo.toml` - Project configuration
- `README.md` - Theory, explanations, and homework
- `src/main.rs` - Code examples

## Weekly Projects

| Week | Project | Skills Combined |
|------|---------|-----------------|
| 1 | Calculator | Variables, Functions, Control Flow, Ownership, Borrowing |
| 2 | Task Manager | Structs, Enums, Error Handling, Collections, Generics, Traits |
| 3 | Todo CLI App | All concepts including Lifetimes, Closures, Iterators, Smart Pointers |

## Learning Tips

1. **Code every day** - Consistency is key
2. **Type the code** - Don't just read, write it yourself
3. **Complete the homework** - TODOs in each README.md files
4. **Read compiler errors** - Rust's error messages are helpful
5. **Ask questions** - Join the Rust community

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust Playground](https://play.rust-lang.org/)

## Contributing

1. Fork this repository
2. Create a new branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -m 'Add new feature'`)
4. Push your branch (`git push origin feature/new-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License.

---

**Let's get started! 🦀**

One step each day, learn Rust in 21 days!

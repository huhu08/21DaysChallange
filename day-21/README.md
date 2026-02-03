# Day 21: Final Project - Todo CLI Application

## Congratulations!

You've reached the final day of the 21 Days of Rust Challenge! Today we build a complete command-line Todo application that combines everything you've learned throughout this journey.

## Project Overview

This final project demonstrates:

- **Structs & Enums** (Day 8-9): Task, Priority, Status
- **Error Handling** (Day 10): Custom TodoError with Result
- **Collections** (Day 11): HashMap for task storage
- **Generics & Traits** (Day 12-13): Display trait implementations
- **Lifetimes** (Day 15): Reference handling in search
- **Closures & Iterators** (Day 16-17): Filtering and mapping tasks
- **File I/O**: Saving and loading tasks

## Features

### Core Features (Implemented)
- Add tasks with title, priority, and tags
- Mark tasks as completed
- List all tasks or filter by status
- Search tasks by title
- View statistics (completion rate)
- Save tasks to file

### Code Highlights

#### Enums for Type Safety
```rust
enum Priority { Low, Medium, High }
enum Status { Pending, InProgress, Completed }
```

#### Custom Error Type
```rust
enum TodoError {
    TaskNotFound(u32),
    IoError(io::Error),
}

impl From<io::Error> for TodoError {
    fn from(error: io::Error) -> Self {
        TodoError::IoError(error)
    }
}
```

#### Iterator-based Search
```rust
fn search(&self, query: &str) -> Vec<&Task> {
    let q = query.to_lowercase();
    self.tasks
        .values()
        .filter(|t| t.title.to_lowercase().contains(&q))
        .collect()
}
```

#### Display Trait for Pretty Printing
```rust
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = match self.status {
            Status::Pending => "[ ]",
            Status::InProgress => "[~]",
            Status::Completed => "[x]",
        };
        write!(f, "{} #{}: {} [{}]", icon, self.id, self.title, self.priority)
    }
}
```

## Run the Project

```bash
cd day-21
cargo run
```

## Sample Output

```
=== Day 21: Final Project - Todo CLI Application ===

--- Creating Tasks ---
Added: [ ] #1: Learn Rust basics [High] [learning]
Added: [ ] #2: Complete Day 21 project [High] [learning]
Added: [ ] #3: Review ownership concepts [Medium] [learning]

--- Completing Tasks ---
Completed: [x] #1: Learn Rust basics [High] [learning]

--- Statistics ---
Total: 5, Completed: 2, Pending: 3
Completion rate: 40.0%
```

## Homework: Extend the Project

### Exercise 1: Add Due Dates
Use the `chrono` crate for date handling:

```toml
[dependencies]
chrono = "0.4"
```

```rust
use chrono::{NaiveDate, Local};

struct Task {
    // ... existing fields
    due_date: Option<NaiveDate>,
}

impl Task {
    fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            due < Local::now().date_naive()
        } else {
            false
        }
    }
}
```

### Exercise 2: Interactive CLI
Use `clap` for command-line argument parsing:

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String, priority: String },
    Complete { id: u32 },
    List,
}
```

### Exercise 3: Database Persistence
Use `rusqlite` for SQLite database:

```toml
[dependencies]
rusqlite = "0.29"
```

### Exercise 4: REST API
Create a web API with `actix-web`:

```toml
[dependencies]
actix-web = "4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## What You've Learned

### Week 0-1: Foundations
| Day | Topic |
|-----|-------|
| 0 | Introduction & Setup |
| 1 | Variables & Mutability |
| 2 | Data Types |
| 3 | Functions |
| 4 | Control Flow |
| 5 | Ownership |
| 6 | Borrowing & Slices |
| 7 | Week 1 Project: Calculator |

### Week 2: Core Concepts
| Day | Topic |
|-----|-------|
| 8 | Structs |
| 9 | Enums & Pattern Matching |
| 10 | Error Handling |
| 11 | Collections |
| 12 | Generics |
| 13 | Traits |
| 14 | Week 2 Project: Task Manager |

### Week 3: Advanced Topics
| Day | Topic |
|-----|-------|
| 15 | Lifetimes |
| 16 | Closures |
| 17 | Iterators |
| 18 | Smart Pointers |
| 19 | Concurrency |
| 20 | Macros |
| 21 | Final Project |

## Next Steps

Continue your Rust journey:

- [The Rust Book](https://doc.rust-lang.org/book/) - Official comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn through examples
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises
- [Exercism Rust Track](https://exercism.org/tracks/rust) - Practice problems
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust) - Curated resources

## Congratulations!

```
+-------------------------------------------------------+
|  Congratulations on completing 21 Days of Rust!       |
|                                                       |
|  You've built a solid foundation in:                  |
|  ✓ Memory safety without garbage collection           |
|  ✓ Zero-cost abstractions                             |
|  ✓ Fearless concurrency                               |
|  ✓ Expressive type system                             |
|                                                       |
|  Keep coding in Rust! 🦀                              |
+-------------------------------------------------------+
```

---

[← Day 20: Macros](../day-20/README.md) | [Home](../README.md)

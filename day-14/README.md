# Day 14: Week 2 Project - Task Manager

## Overview

Congratulations on completing Week 2! Today you'll build a command-line task manager that combines everything you've learned:

- **Structs** (Day 8)
- **Enums** (Day 9)
- **Error Handling** (Day 10)
- **Collections** (Day 11)
- **Generics** (Day 12)
- **Traits** (Day 13)

## Project Description

Build an interactive task management system with the following features:

### Core Features (Implemented)
1. **Add Tasks**: Create tasks with title, priority, and category
2. **List Tasks**: View all tasks with status icons
3. **Complete Tasks**: Mark tasks as done
4. **Delete Tasks**: Remove tasks
5. **Filter**: View tasks by priority or category
6. **Statistics**: See completion rates and breakdowns

### Concepts Applied

#### Enums for Type Safety
```rust
enum Priority { Low, Medium, High, Urgent }
enum Status { Pending, InProgress, Completed }
enum Category { Work, Personal, Learning, Health, Finance }
```

#### Custom Error Types
```rust
enum TaskError {
    NotFound(u32),
    AlreadyCompleted(u32),
    InvalidInput(String),
}
```

#### Structs with Methods
```rust
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    status: Status,
}

impl Task {
    fn complete(&mut self) -> Result<(), TaskError> { ... }
}
```

#### Traits for Shared Behavior
```rust
trait Summarizable {
    fn summary(&self) -> String;
}

trait Filterable<T> {
    fn filter_by(&self, criteria: T) -> Vec<&Task>;
}
```

#### Collections (HashMap)
```rust
struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}
```

## Running the Project

```bash
cd day-14
cargo run
```

## Sample Session

```
=== Day 14: Week 2 Project - Task Manager ===

--- Task Manager Menu ---
1. Add Task
2. List All Tasks
3. Complete Task
4. Delete Task
5. Filter by Priority
6. Filter by Category
7. Show Statistics
8. Exit

Choose an option (1-8): 1

--- Add New Task ---
Title: Learn Rust lifetimes
Priority: 1=Low, 2=Medium, 3=High, 4=Urgent
Choose (1-4): 3
Category: 1=Work, 2=Personal, 3=Learning, 4=Health, 5=Finance
Choose (1-5): 3
Task #1 added successfully!
```

## Homework

Extend the task manager with these additional features:

### Exercise 1: Due Dates
Add due date support using the `chrono` crate:

```toml
# Add to Cargo.toml
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
        // Check if task is past due
    }
}
```

### Exercise 2: JSON Persistence
Save and load tasks using `serde`:

```toml
# Add to Cargo.toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Task { ... }

impl TaskManager {
    fn save_to_file(&self, path: &str) -> Result<(), io::Error>
    fn load_from_file(path: &str) -> Result<Self, io::Error>
}
```

### Exercise 3: Subtasks
Allow tasks to have subtasks:

```rust
struct Task {
    // ... existing fields
    subtasks: Vec<Subtask>,
}

struct Subtask {
    title: String,
    completed: bool,
}

impl Task {
    fn completion_percentage(&self) -> f64 {
        // Calculate based on subtasks
    }
}
```

### Exercise 4: Search Functionality
Add ability to search tasks:

```rust
impl TaskManager {
    fn search(&self, query: &str) -> Vec<&Task> {
        // Search in title and description
    }
}
```

### Exercise 5: Undo/Redo
Implement operation history:

```rust
enum Operation {
    Add(Task),
    Complete(u32),
    Delete(Task),
}

struct TaskManager {
    // ... existing fields
    history: Vec<Operation>,
}

impl TaskManager {
    fn undo(&mut self) -> Result<(), TaskError>
    fn redo(&mut self) -> Result<(), TaskError>
}
```

## Week 2 Summary

| Day | Topic | Key Concepts |
|-----|-------|--------------|
| 8 | Structs | Fields, methods, associated functions |
| 9 | Enums | Variants, pattern matching, Option |
| 10 | Error Handling | Result, ?, custom errors |
| 11 | Collections | Vec, String, HashMap |
| 12 | Generics | Type parameters, bounds |
| 13 | Traits | Shared behavior, impl, dyn |
| 14 | **Project** | Task Manager combining all concepts |

## Navigation

[← Day 13: Traits](../day-13/README.md) | [Home](../README.md) | [Day 15: Lifetimes →](../day-15/README.md)

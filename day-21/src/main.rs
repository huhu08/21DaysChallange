// Day 21: Final Project - Todo CLI Application
// This file contains the final project combining all 21 days of learning

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    println!("=== Day 21: Final Project - Todo CLI Application ===\n");

    let mut app = TodoApp::new();

    // Demo all features
    println!("--- Creating Tasks ---");
    app.add_task("Learn Rust basics", Priority::High, "learning");
    app.add_task("Complete Day 21 project", Priority::High, "learning");
    app.add_task("Review ownership concepts", Priority::Medium, "learning");
    app.add_task("Exercise", Priority::Low, "health");
    app.add_task("Read a book", Priority::Low, "personal");

    println!("\n--- All Tasks ---");
    app.list_all();

    println!("\n--- Completing Tasks ---");
    app.complete_task(1);
    app.complete_task(4);

    println!("\n--- Tasks by Status ---");
    println!("Pending:");
    app.list_by_status(Status::Pending);
    println!("\nCompleted:");
    app.list_by_status(Status::Completed);

    println!("\n--- Search ---");
    let results = app.search("Rust");
    println!("Search 'Rust': {} result(s)", results.len());
    for task in results {
        println!("  {}", task);
    }

    println!("\n--- Statistics ---");
    app.print_stats();

    println!("\n--- Save and Load ---");
    let filename = "/tmp/todos.txt";
    match app.save_to_file(filename) {
        Ok(_) => println!("Saved to {}", filename),
        Err(e) => println!("Error saving: {}", e),
    }

    println!("\n=== Congratulations! You've completed 21 Days of Rust! ===");
    print_completion_message();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Pending,
    InProgress,
    Completed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    status: Status,
    tags: Vec<String>,
}

impl Task {
    fn new(id: u32, title: &str, priority: Priority, tags: Vec<String>) -> Self {
        Task {
            id,
            title: title.to_string(),
            priority,
            status: Status::Pending,
            tags,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = match self.status {
            Status::Pending => "[ ]",
            Status::InProgress => "[~]",
            Status::Completed => "[x]",
        };
        let tags = if self.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", self.tags.join(", "))
        };
        write!(
            f,
            "{} #{}: {} [{}]{}",
            icon, self.id, self.title, self.priority, tags
        )
    }
}

#[derive(Debug)]
enum TodoError {
    TaskNotFound(u32),
    IoError(io::Error),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::TaskNotFound(id) => write!(f, "Task #{} not found", id),
            TodoError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<io::Error> for TodoError {
    fn from(error: io::Error) -> Self {
        TodoError::IoError(error)
    }
}

struct TodoApp {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TodoApp {
    fn new() -> Self {
        TodoApp {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: &str, priority: Priority, tag: &str) -> u32 {
        let tags = if tag.is_empty() {
            vec![]
        } else {
            vec![tag.to_string()]
        };
        let task = Task::new(self.next_id, title, priority, tags);
        println!("Added: {}", task);
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
        self.next_id - 1
    }

    fn complete_task(&mut self, id: u32) -> Result<(), TodoError> {
        match self.tasks.get_mut(&id) {
            Some(task) => {
                task.status = Status::Completed;
                println!("Completed: {}", task);
                Ok(())
            }
            None => Err(TodoError::TaskNotFound(id)),
        }
    }

    fn list_all(&self) {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        tasks.sort_by_key(|t| t.id);
        for task in tasks {
            println!("  {}", task);
        }
    }

    fn list_by_status(&self, status: Status) {
        for task in self.tasks.values().filter(|t| t.status == status) {
            println!("  {}", task);
        }
    }

    fn search(&self, query: &str) -> Vec<&Task> {
        let q = query.to_lowercase();
        self.tasks
            .values()
            .filter(|t| t.title.to_lowercase().contains(&q))
            .collect()
    }

    fn print_stats(&self) {
        let total = self.tasks.len();
        let completed = self
            .tasks
            .values()
            .filter(|t| t.status == Status::Completed)
            .count();
        println!(
            "Total: {}, Completed: {}, Pending: {}",
            total,
            completed,
            total - completed
        );
        if total > 0 {
            println!(
                "Completion rate: {:.1}%",
                (completed as f64 / total as f64) * 100.0
            );
        }
    }

    fn save_to_file(&self, filename: &str) -> Result<(), TodoError> {
        let mut file = File::create(filename)?;
        for task in self.tasks.values() {
            writeln!(
                file,
                "{}|{}|{}|{}|{}",
                task.id,
                task.title,
                task.priority,
                task.status,
                task.tags.join(",")
            )?;
        }
        Ok(())
    }
}

fn print_completion_message() {
    println!(
        "
+-------------------------------------------------------+
|  Congratulations on completing 21 Days of Rust!       |
|                                                       |
|  You've learned:                                      |
|  - Variables, Data Types, Functions                   |
|  - Ownership, Borrowing, Lifetimes                    |
|  - Structs, Enums, Pattern Matching                   |
|  - Error Handling, Collections                        |
|  - Generics, Traits                                   |
|  - Closures, Iterators                                |
|  - Smart Pointers, Concurrency                        |
|  - Modules, Crates, Macros                            |
|                                                       |
|  Keep practicing and building projects!               |
+-------------------------------------------------------+
    "
    );
}

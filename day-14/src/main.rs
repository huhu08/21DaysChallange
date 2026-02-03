// Day 14: Week 2 Project - Task Manager
// This file contains the Week 2 project for the 21 Days of Rust Challenge
// Build a task manager using Structs, Enums, Error Handling, Collections, Generics, and Traits!

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::io::{self, Write};

fn main() {
    println!("=== Day 14: Week 2 Project - Task Manager ===\n");
    println!(
        "This project combines: Structs, Enums, Error Handling, Collections, Generics, and Traits\n"
    );

    let mut manager = TaskManager::new();

    loop {
        print_menu();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => add_task_interactive(&mut manager),
            "2" => list_tasks(&manager),
            "3" => complete_task_interactive(&mut manager),
            "4" => delete_task_interactive(&mut manager),
            "5" => filter_by_priority(&manager),
            "6" => filter_by_category(&manager),
            "7" => show_statistics(&manager),
            "8" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
        println!();
    }
}

fn print_menu() {
    println!("\n--- Task Manager Menu ---");
    println!("1. Add Task");
    println!("2. List All Tasks");
    println!("3. Complete Task");
    println!("4. Delete Task");
    println!("5. Filter by Priority");
    println!("6. Filter by Category");
    println!("7. Show Statistics");
    println!("8. Exit");
    print!("\nChoose an option (1-8): ");
    io::stdout().flush().unwrap();
}

// ============================================================
// ENUMS
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
            Priority::Urgent => write!(f, "URGENT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Status {
    Pending,
    InProgress,
    Completed,
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    Work,
    Personal,
    Learning,
    Health,
    Finance,
}

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Work => write!(f, "Work"),
            Category::Personal => write!(f, "Personal"),
            Category::Learning => write!(f, "Learning"),
            Category::Health => write!(f, "Health"),
            Category::Finance => write!(f, "Finance"),
        }
    }
}

// ============================================================
// ERROR HANDLING
// ============================================================

#[derive(Debug)]
enum TaskError {
    NotFound(u32),
    AlreadyCompleted(u32),
    InvalidInput(String),
}

impl Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::NotFound(id) => write!(f, "Task #{} not found", id),
            TaskError::AlreadyCompleted(id) => write!(f, "Task #{} is already completed", id),
            TaskError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

// ============================================================
// STRUCTS
// ============================================================

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    description: Option<String>,
    priority: Priority,
    category: Category,
    status: Status,
}

impl Task {
    fn new(id: u32, title: String, priority: Priority, category: Category) -> Self {
        Task {
            id,
            title,
            description: None,
            priority,
            category,
            status: Status::Pending,
        }
    }

    fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    fn complete(&mut self) -> Result<(), TaskError> {
        if self.status == Status::Completed {
            Err(TaskError::AlreadyCompleted(self.id))
        } else {
            self.status = Status::Completed;
            Ok(())
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_icon = match self.status {
            Status::Pending => "[ ]",
            Status::InProgress => "[~]",
            Status::Completed => "[✓]",
        };
        write!(
            f,
            "{} #{}: {} [{}] [{}]",
            status_icon, self.id, self.title, self.priority, self.category
        )?;
        if let Some(ref desc) = self.description {
            write!(f, "\n      {}", desc)?;
        }
        Ok(())
    }
}

// ============================================================
// TRAITS
// ============================================================

trait Summarizable {
    fn summary(&self) -> String;
}

impl Summarizable for Task {
    fn summary(&self) -> String {
        format!("#{}: {} ({})", self.id, self.title, self.status)
    }
}

trait Filterable<T> {
    fn filter_by(&self, criteria: T) -> Vec<&Task>;
}

// ============================================================
// TASK MANAGER (Uses Collections & Generics)
// ============================================================

struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String, priority: Priority, category: Category) -> u32 {
        let id = self.next_id;
        let task = Task::new(id, title, priority, category);
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }

    fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }

    fn complete_task(&mut self, id: u32) -> Result<(), TaskError> {
        match self.tasks.get_mut(&id) {
            Some(task) => task.complete(),
            None => Err(TaskError::NotFound(id)),
        }
    }

    fn delete_task(&mut self, id: u32) -> Result<Task, TaskError> {
        self.tasks.remove(&id).ok_or(TaskError::NotFound(id))
    }

    fn list_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn count(&self) -> usize {
        self.tasks.len()
    }
}

// Implement Filterable for Priority
impl Filterable<Priority> for TaskManager {
    fn filter_by(&self, priority: Priority) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|t| t.priority == priority)
            .collect()
    }
}

// Implement Filterable for Category
impl Filterable<Category> for TaskManager {
    fn filter_by(&self, category: Category) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|t| t.category == category)
            .collect()
    }
}

// Implement Filterable for Status
impl Filterable<Status> for TaskManager {
    fn filter_by(&self, status: Status) -> Vec<&Task> {
        self.tasks.values().filter(|t| t.status == status).collect()
    }
}

// ============================================================
// STATISTICS (Using Generics)
// ============================================================

struct Statistics {
    total: usize,
    completed: usize,
    pending: usize,
    in_progress: usize,
    by_priority: HashMap<Priority, usize>,
    by_category: HashMap<Category, usize>,
}

impl TaskManager {
    fn statistics(&self) -> Statistics {
        let mut stats = Statistics {
            total: self.tasks.len(),
            completed: 0,
            pending: 0,
            in_progress: 0,
            by_priority: HashMap::new(),
            by_category: HashMap::new(),
        };

        for task in self.tasks.values() {
            match task.status {
                Status::Completed => stats.completed += 1,
                Status::Pending => stats.pending += 1,
                Status::InProgress => stats.in_progress += 1,
            }

            *stats.by_priority.entry(task.priority).or_insert(0) += 1;
            *stats.by_category.entry(task.category).or_insert(0) += 1;
        }

        stats
    }
}

// ============================================================
// INTERACTIVE FUNCTIONS
// ============================================================

fn add_task_interactive(manager: &mut TaskManager) {
    println!("\n--- Add New Task ---");

    print!("Title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim().to_string();

    println!("Priority: 1=Low, 2=Medium, 3=High, 4=Urgent");
    print!("Choose (1-4): ");
    io::stdout().flush().unwrap();
    let mut priority_input = String::new();
    io::stdin().read_line(&mut priority_input).unwrap();
    let priority = match priority_input.trim() {
        "1" => Priority::Low,
        "2" => Priority::Medium,
        "3" => Priority::High,
        "4" => Priority::Urgent,
        _ => Priority::Medium,
    };

    println!("Category: 1=Work, 2=Personal, 3=Learning, 4=Health, 5=Finance");
    print!("Choose (1-5): ");
    io::stdout().flush().unwrap();
    let mut category_input = String::new();
    io::stdin().read_line(&mut category_input).unwrap();
    let category = match category_input.trim() {
        "1" => Category::Work,
        "2" => Category::Personal,
        "3" => Category::Learning,
        "4" => Category::Health,
        "5" => Category::Finance,
        _ => Category::Personal,
    };

    let id = manager.add_task(title, priority, category);
    println!("Task #{} added successfully!", id);
}

fn list_tasks(manager: &TaskManager) {
    println!("\n--- All Tasks ---");
    let tasks = manager.list_all();
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            println!("{}", task);
        }
    }
}

fn complete_task_interactive(manager: &mut TaskManager) {
    print!("Enter task ID to complete: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<u32>() {
        Ok(id) => match manager.complete_task(id) {
            Ok(()) => println!("Task #{} completed!", id),
            Err(e) => println!("Error: {}", e),
        },
        Err(_) => println!("Invalid ID"),
    }
}

fn delete_task_interactive(manager: &mut TaskManager) {
    print!("Enter task ID to delete: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<u32>() {
        Ok(id) => match manager.delete_task(id) {
            Ok(task) => println!("Deleted: {}", task.summary()),
            Err(e) => println!("Error: {}", e),
        },
        Err(_) => println!("Invalid ID"),
    }
}

fn filter_by_priority(manager: &TaskManager) {
    println!("Priority: 1=Low, 2=Medium, 3=High, 4=Urgent");
    print!("Choose (1-4): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let priority = match input.trim() {
        "1" => Priority::Low,
        "2" => Priority::Medium,
        "3" => Priority::High,
        "4" => Priority::Urgent,
        _ => return println!("Invalid priority"),
    };

    let tasks: Vec<&Task> = manager.filter_by(priority);
    println!("\n--- {} Priority Tasks ---", priority);
    for task in tasks {
        println!("{}", task);
    }
}

fn filter_by_category(manager: &TaskManager) {
    println!("Category: 1=Work, 2=Personal, 3=Learning, 4=Health, 5=Finance");
    print!("Choose (1-5): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let category = match input.trim() {
        "1" => Category::Work,
        "2" => Category::Personal,
        "3" => Category::Learning,
        "4" => Category::Health,
        "5" => Category::Finance,
        _ => return println!("Invalid category"),
    };

    let tasks: Vec<&Task> = manager.filter_by(category);
    println!("\n--- {} Tasks ---", category);
    for task in tasks {
        println!("{}", task);
    }
}

fn show_statistics(manager: &TaskManager) {
    let stats = manager.statistics();

    println!("\n--- Statistics ---");
    println!("Total tasks: {}", stats.total);
    println!("Completed: {}", stats.completed);
    println!("In Progress: {}", stats.in_progress);
    println!("Pending: {}", stats.pending);

    if stats.total > 0 {
        let completion_rate = (stats.completed as f64 / stats.total as f64) * 100.0;
        println!("Completion rate: {:.1}%", completion_rate);
    }

    println!("\nBy Priority:");
    for (priority, count) in &stats.by_priority {
        println!("  {}: {}", priority, count);
    }

    println!("\nBy Category:");
    for (category, count) in &stats.by_category {
        println!("  {}: {}", category, count);
    }
}

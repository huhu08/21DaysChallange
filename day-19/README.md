# Day 19: Concurrency

## What Will We Learn Today?
- Creating and managing threads
- Message passing with channels
- Shared state with Mutex and Arc
- Thread-safe patterns

## Creating Threads

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from a thread!");
});

handle.join().unwrap();  // Wait for thread to finish
```

## Move Closures

Transfer ownership to threads with `move`:

```rust
let data = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("{:?}", data);  // data is moved here
});
```

## Message Passing with Channels

Channels provide a way to send messages between threads:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(String::from("hello")).unwrap();
});

let received = rx.recv().unwrap();
```

### Multiple Producers

```rust
let (tx, rx) = mpsc::channel();
let tx2 = tx.clone();

thread::spawn(move || { tx.send("from 1").unwrap(); });
thread::spawn(move || { tx2.send("from 2").unwrap(); });

for msg in rx {
    println!("{}", msg);
}
```

## Shared State with Mutex

Mutex provides mutual exclusion for shared data:

```rust
use std::sync::Mutex;

let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap();
    *num = 6;
}  // Lock released here
```

## Arc for Multi-threaded Sharing

`Arc` (Atomic Reference Counting) allows sharing across threads:

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..5 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

## RwLock for Read-Heavy Workloads

Multiple readers OR one writer:

```rust
use std::sync::RwLock;

let lock = RwLock::new(5);

// Multiple readers allowed
let r1 = lock.read().unwrap();
let r2 = lock.read().unwrap();

// Writer blocks all others
let mut w = lock.write().unwrap();
*w += 1;
```

## Thread Safety Rules

| Type | Use Case |
|------|----------|
| `Rc<T>` | Single-threaded reference counting |
| `Arc<T>` | Multi-threaded reference counting |
| `RefCell<T>` | Single-threaded interior mutability |
| `Mutex<T>` | Multi-threaded interior mutability |
| `RwLock<T>` | Multi-threaded, read-heavy workloads |

## Send and Sync Traits

- `Send`: Type can be transferred between threads
- `Sync`: Type can be referenced from multiple threads

Most types implement both automatically.

## Run the Examples

```bash
cd day-19
cargo run
```

## Homework

### Homework 1: Thread Pool
Create a simple thread pool:

```rust
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool { /* ... */ }
    
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    { /* ... */ }
}
```

### Homework 2: Parallel Map
Implement a parallel map function:

```rust
fn parallel_map<T, U, F>(items: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + Clone + 'static,
{
    // Distribute items across threads
    // Collect results in order
}
```

### Homework 3: Producer-Consumer Queue
Create a bounded producer-consumer queue:

```rust
struct BoundedQueue<T> {
    // Use Arc<Mutex<VecDeque<T>>> or channels
}

impl<T> BoundedQueue<T> {
    fn new(capacity: usize) -> Self { /* ... */ }
    fn push(&self, item: T) { /* blocks if full */ }
    fn pop(&self) -> T { /* blocks if empty */ }
}
```

---

[← Day 18: Smart Pointers](../day-18/README.md) | [Home](../README.md) | [Day 20: Macros →](../day-20/README.md)

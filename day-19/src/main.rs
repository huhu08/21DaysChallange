// Day 19: Concurrency
// This file contains examples for Day 19 of the 21 Days of Rust Challenge

use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Day 19: Concurrency ===\n");

    // Example 1: Creating threads
    println!("Example 1: Creating Threads");
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  Spawned thread: count {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("  Main thread: count {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    handle.join().unwrap(); // Wait for spawned thread to finish
    println!();

    // Example 2: Move closure - transferring ownership
    println!("Example 2: Move Closure");
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("  Vector in thread: {:?}", v);
    });

    handle.join().unwrap();
    // v is no longer accessible here - it was moved to the thread
    println!();

    // Example 3: Channels - message passing
    println!("Example 3: Channels (mpsc)");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("Hello from thread!");
        tx.send(message).unwrap();
        // message is moved, can't use it here
    });

    let received = rx.recv().unwrap();
    println!("  Received: {}", received);
    println!();

    // Example 4: Sending multiple messages
    println!("Example 4: Multiple Messages");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ];

        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // rx acts as an iterator
    for received in rx {
        println!("  Got: {}", received);
    }
    println!();

    // Example 5: Multiple producers
    println!("Example 5: Multiple Producers");
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx.send(String::from("Producer 1: Hello")).unwrap();
    });

    thread::spawn(move || {
        tx2.send(String::from("Producer 2: Hi")).unwrap();
    });

    for received in rx {
        println!("  {}", received);
    }
    println!();

    // Example 6: Mutex for shared state
    println!("Example 6: Mutex");
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("  Modified value: {}", num);
    } // Lock is released here

    println!("  Final value: {:?}", m);
    println!();

    // Example 7: Arc<Mutex<T>> for thread-safe shared state
    println!("Example 7: Arc<Mutex<T>>");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("  Thread {} incremented counter to {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  Final counter: {}", *counter.lock().unwrap());
    println!();

    // Example 8: RwLock for read-heavy workloads
    println!("Example 8: RwLock");
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // Multiple readers
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_guard = data.read().unwrap();
            println!("  Reader {}: {:?}", i, *read_guard);
        });
        handles.push(handle);
    }

    // One writer
    {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_guard = data.write().unwrap();
            write_guard.push(4);
            println!("  Writer added 4");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("  Final data: {:?}", *data.read().unwrap());
    println!();

    // Example 9: Thread-safe counter with return values
    println!("Example 9: Returning Values from Threads");
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(50));
                i * i
            })
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    println!("  Squares: {:?}", results);
    println!();

    // Example 10: Parallel computation
    println!("Example 10: Parallel Sum");
    let numbers: Vec<i32> = (1..=100).collect();
    let chunk_size = 25;

    let sum = parallel_sum(&numbers, chunk_size);
    println!("  Sum of 1-100: {}", sum);

    // Verify
    let expected: i32 = (1..=100).sum();
    println!("  Expected: {}", expected);
    println!();

    println!("=== End of Day 19 Examples ===");
}

// Parallel sum using multiple threads
fn parallel_sum(numbers: &[i32], chunk_size: usize) -> i32 {
    let chunks: Vec<_> = numbers.chunks(chunk_size).collect();
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            let chunk: Vec<i32> = chunk.to_vec();
            thread::spawn(move || chunk.iter().sum::<i32>())
        })
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

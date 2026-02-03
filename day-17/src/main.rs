// Day 17: Iterators
// This file contains examples for Day 17 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 17: Iterators ===\n");

    // Example 1: Basic iterator
    println!("Example 1: Basic Iterator");
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    println!("First next(): {:?}", iter.next());
    println!("Second next(): {:?}", iter.next());
    println!("Third next(): {:?}", iter.next());
    println!("Fourth next(): {:?}", iter.next()); // None
    println!();

    // Example 2: Iterator types
    println!("Example 2: Iterator Types");
    let v = vec![1, 2, 3];

    // iter() - iterates over &T
    print!("iter() (references): ");
    for val in v.iter() {
        print!("{} ", val);
    }
    println!();

    // iter_mut() - iterates over &mut T
    let mut v2 = vec![1, 2, 3];
    for val in v2.iter_mut() {
        *val *= 2;
    }
    println!("After iter_mut() doubling: {:?}", v2);

    // into_iter() - takes ownership, iterates over T
    let v3 = vec![1, 2, 3];
    let sum: i32 = v3.into_iter().sum();
    println!("into_iter() sum: {}", sum);
    // v3 is no longer available
    println!();

    // Example 3: map - transform each element
    println!("Example 3: map");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);
    println!();

    // Example 4: filter - keep matching elements
    println!("Example 4: filter");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).cloned().collect();
    println!("Numbers: {:?}", numbers);
    println!("Evens: {:?}", evens);
    println!();

    // Example 5: Chaining iterators
    println!("Example 5: Chaining");
    let result: Vec<i32> = (1..=10)
        .filter(|x| x % 2 == 0) // Keep evens
        .map(|x| x * x) // Square them
        .collect();
    println!("Even squares from 1-10: {:?}", result);
    println!();

    // Example 6: Consuming adaptors
    println!("Example 6: Consuming Adaptors");
    let nums = vec![1, 2, 3, 4, 5];
    println!("Numbers: {:?}", nums);
    println!("sum(): {}", nums.iter().sum::<i32>());
    println!("product(): {}", nums.iter().product::<i32>());
    println!("max(): {:?}", nums.iter().max());
    println!("min(): {:?}", nums.iter().min());
    println!("count(): {}", nums.iter().count());
    println!();

    // Example 7: fold - reduce to single value
    println!("Example 7: fold");
    let nums = vec![1, 2, 3, 4, 5];

    // Sum using fold
    let sum = nums.iter().fold(0, |acc, x| acc + x);
    println!("fold sum: {}", sum);

    // Product using fold
    let product = nums.iter().fold(1, |acc, x| acc * x);
    println!("fold product: {}", product);

    // Build string using fold
    let joined = nums.iter().fold(String::new(), |acc, x| {
        if acc.is_empty() {
            x.to_string()
        } else {
            format!("{}, {}", acc, x)
        }
    });
    println!("fold joined: {}", joined);
    println!();

    // Example 8: find, any, all
    println!("Example 8: find, any, all");
    let nums = vec![1, 2, 3, 4, 5];
    println!("find(|x| x > 3): {:?}", nums.iter().find(|&&x| x > 3));
    println!("any(|x| x > 4): {}", nums.iter().any(|&x| x > 4));
    println!("all(|x| x > 0): {}", nums.iter().all(|&x| x > 0));
    println!("all(|x| x > 3): {}", nums.iter().all(|&x| x > 3));
    println!();

    // Example 9: take, skip, step_by
    println!("Example 9: take, skip, step_by");
    let nums: Vec<i32> = (1..=10).collect();
    println!("Original: {:?}", nums);
    println!("take(3): {:?}", nums.iter().take(3).collect::<Vec<_>>());
    println!("skip(7): {:?}", nums.iter().skip(7).collect::<Vec<_>>());
    println!(
        "step_by(2): {:?}",
        nums.iter().step_by(2).collect::<Vec<_>>()
    );
    println!();

    // Example 10: enumerate and zip
    println!("Example 10: enumerate and zip");
    let fruits = vec!["apple", "banana", "cherry"];

    println!("enumerate:");
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  {}: {}", i, fruit);
    }

    let prices = vec![1.50, 0.75, 2.00];
    let zipped: Vec<_> = fruits.iter().zip(prices.iter()).collect();
    println!("zip with prices: {:?}", zipped);
    println!();

    // Example 11: flatten
    println!("Example 11: flatten");
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("Flattened: {:?}", flat);
    println!();

    // Example 12: Custom iterator
    println!("Example 12: Custom Iterator");
    let counter = Counter::new(5);
    let result: Vec<u32> = counter.collect();
    println!("Counter: {:?}", result);

    // Using custom iterator with adapters
    let sum: u32 = Counter::new(5).filter(|x| x % 2 == 0).sum();
    println!("Sum of even counts: {}", sum);
    println!();

    // Example 13: Fibonacci iterator
    println!("Example 13: Fibonacci Iterator");
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("First 10 Fibonacci: {:?}", fibs);
    println!();

    println!("=== End of Day 17 Examples ===");
}

// Custom iterator - Counter
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Fibonacci iterator
struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

# Day 16: Closures

## What Will We Learn Today?
- Closure syntax
- Capturing environment
- Fn, FnMut, and FnOnce traits
- Closures as parameters and return values
- Practical closure patterns

## What Are Closures?

Closures are anonymous functions that can capture their environment.

```rust
let add_one = |x| x + 1;
println!("{}", add_one(5));  // 6
```

## Closure Syntax

```rust
// Fully annotated
let add = |x: i32, y: i32| -> i32 { x + y };

// Type inference
let add = |x, y| x + y;

// Single expression (no braces needed)
let double = |x| x * 2;
```

## Capturing Environment

### Immutable Borrow (Fn)
```rust
let x = 4;
let equal_to_x = |z| z == x;  // Borrows x
println!("{}", equal_to_x(4));  // true
println!("{}", x);  // x still accessible
```

### Mutable Borrow (FnMut)
```rust
let mut count = 0;
let mut increment = || {
    count += 1;  // Mutably borrows count
};
increment();
increment();
```

### Taking Ownership (FnOnce)
```rust
let data = vec![1, 2, 3];
let consume = move || {
    println!("{:?}", data);  // data is moved
};
// data is no longer accessible here
```

## Closure Traits

| Trait | Can Call | Can Mutate | Takes Ownership |
|-------|----------|------------|-----------------|
| `Fn` | Multiple times | No | No |
| `FnMut` | Multiple times | Yes | No |
| `FnOnce` | Once | Yes | Yes |

## Closures as Parameters

```rust
fn apply<F>(x: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

let result = apply(5, |x| x * 2);
```

## Returning Closures

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

let add_five = make_adder(5);
println!("{}", add_five(3));  // 8
```

## Closures with Iterators

```rust
let numbers = vec![1, 2, 3, 4, 5];

// map - transform each element
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .collect();

// filter - keep elements matching predicate
let evens: Vec<i32> = numbers.iter()
    .filter(|x| *x % 2 == 0)
    .cloned()
    .collect();
```

## Storing Closures

```rust
struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    calculation: T,
    value: Option<i32>,
}
```

## Run the Examples

```bash
cd day-16
cargo run
```

## Homework

### Homework 1: Transform Function
Create a generic transform function:

```rust
fn transform<T, F>(items: Vec<T>, f: F) -> Vec<T>
where
    F: Fn(T) -> T,
{
    // Apply f to each element
}

// Usage
let numbers = vec![1, 2, 3];
let doubled = transform(numbers, |x| x * 2);
```

### Homework 2: Event Emitter
Create an event emitter pattern:

```rust
struct EventEmitter<T> {
    handlers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> EventEmitter<T> {
    fn new() -> Self { /* ... */ }
    fn on<F>(&mut self, handler: F) where F: Fn(&T) + 'static { /* ... */ }
    fn emit(&self, event: &T) { /* ... */ }
}
```

### Homework 3: Pipeline Builder
Create a fluent pipeline API:

```rust
struct Pipeline<T> {
    value: T,
}

impl<T> Pipeline<T> {
    fn new(value: T) -> Self { /* ... */ }
    fn pipe<F, U>(self, f: F) -> Pipeline<U> 
    where F: FnOnce(T) -> U { /* ... */ }
    fn execute(self) -> T { /* ... */ }
}

// Usage
let result = Pipeline::new(5)
    .pipe(|x| x * 2)
    .pipe(|x| x + 1)
    .execute();  // 11
```

---

[← Day 15: Lifetimes](../day-15/README.md) | [Home](../README.md) | [Day 17: Iterators →](../day-17/README.md)

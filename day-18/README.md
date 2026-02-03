# Day 18: Smart Pointers

## What Will We Learn Today?
- Box<T> for heap allocation
- Deref and Drop traits
- Rc<T> for reference counting
- RefCell<T> for interior mutability
- Weak<T> to prevent reference cycles

## What Are Smart Pointers?

Smart pointers are data structures that act like pointers but have additional metadata and capabilities. They own the data they point to.

## Box<T> - Heap Allocation

`Box<T>` puts data on the heap instead of the stack:

```rust
let b = Box::new(5);
println!("{}", *b);  // Dereference with *
```

### Use Cases
- Large data you don't want to copy
- Unknown size at compile time
- Recursive types

```rust
// Recursive type needs Box to have known size
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

## Deref Trait

Allows using `*` operator:

```rust
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

### Deref Coercion
Rust automatically converts types through `Deref`:
```rust
fn hello(name: &str) { }

let m = MyBox::new(String::from("Rust"));
hello(&m);  // &MyBox<String> -> &String -> &str
```

## Drop Trait

Called when value goes out of scope:

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping: {}", self.data);
    }
}
```

Early drop with `std::mem::drop`:
```rust
let c = CustomSmartPointer { data: String::from("stuff") };
drop(c);  // Dropped here, not at end of scope
```

## Rc<T> - Reference Counting

Share ownership between multiple parts:

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);  // Increments count
let c = Rc::clone(&a);

println!("Count: {}", Rc::strong_count(&a));  // 3
```

> Note: Rc<T> is only for single-threaded scenarios

## RefCell<T> - Interior Mutability

Mutate data even with immutable references (checked at runtime):

```rust
use std::cell::RefCell;

let data = RefCell::new(5);
*data.borrow_mut() += 10;
println!("{}", data.borrow());
```

### Borrowing Rules (Runtime)
- Many immutable borrows (`borrow()`) OR
- One mutable borrow (`borrow_mut()`)
- Panics if violated at runtime!

## Rc<RefCell<T>> Pattern

Combine Rc and RefCell for shared mutable data:

```rust
let value = Rc::new(RefCell::new(5));

let a = Rc::clone(&value);
let b = Rc::clone(&value);

*a.borrow_mut() += 10;
*b.borrow_mut() += 20;
// value is now 35
```

## Weak<T> - Avoiding Cycles

`Weak<T>` doesn't count towards ownership:

```rust
use std::rc::Weak;

let strong = Rc::new(5);
let weak: Weak<i32> = Rc::downgrade(&strong);

// Upgrade returns Option<Rc<T>>
if let Some(strong_ref) = weak.upgrade() {
    println!("{}", strong_ref);
}
```

Use `Weak` for parent references in tree structures to avoid cycles.

## Summary

| Type | Ownership | Mutability | Thread-safe |
|------|-----------|------------|-------------|
| `Box<T>` | Single | If T is | If T is |
| `Rc<T>` | Multiple | Immutable | No |
| `RefCell<T>` | Single | Interior | No |
| `Arc<T>` | Multiple | Immutable | Yes |

## Run the Examples

```bash
cd day-18
cargo run
```

## Homework

### Homework 1: Doubly-Linked List
Create a doubly-linked list using smart pointers:

```rust
struct DNode<T> {
    value: T,
    next: Option<Rc<RefCell<DNode<T>>>>,
    prev: Option<Weak<RefCell<DNode<T>>>>,
}

struct DList<T> {
    head: Option<Rc<RefCell<DNode<T>>>>,
    tail: Option<Weak<RefCell<DNode<T>>>>,
}
```

### Homework 2: Lazy Cache
Create a cache that computes values lazily:

```rust
struct LazyCache<T, F>
where
    F: Fn() -> T,
{
    computation: F,
    value: RefCell<Option<T>>,
}

impl<T, F> LazyCache<T, F>
where
    F: Fn() -> T,
    T: Clone,
{
    fn get(&self) -> T {
        // Compute only once, then cache
    }
}
```

### Homework 3: Tree with Shared Children
Create a tree where nodes can have multiple parents:

```rust
struct TreeNode<T> {
    value: T,
    parents: RefCell<Vec<Weak<TreeNode<T>>>>,
    children: RefCell<Vec<Rc<TreeNode<T>>>>,
}
```

---

[← Day 17: Iterators](../day-17/README.md) | [Home](../README.md) | [Day 19: Concurrency →](../day-19/README.md)

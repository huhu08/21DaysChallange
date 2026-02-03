// Day 18: Smart Pointers
// This file contains examples for Day 18 of the 21 Days of Rust Challenge

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    println!("=== Day 18: Smart Pointers ===\n");

    // Example 1: Box<T> - Heap allocation
    println!("Example 1: Box<T>");
    let b = Box::new(5);
    println!("b = {}", b);
    println!("*b = {}", *b); // Dereference

    // Box is useful for large data
    let large_array = Box::new([0u8; 1000000]);
    println!("Large array on heap, first element: {}", large_array[0]);
    println!();

    // Example 2: Recursive types with Box
    println!("Example 2: Recursive Type (Cons List)");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    print!("List: ");
    print_list(&list);
    println!();

    // Example 3: Deref trait
    println!("Example 3: Deref Trait");
    let x = 5;
    let y = Box::new(x);
    let z = MyBox::new(x);

    println!("x = {}", x);
    println!("*y (Box) = {}", *y);
    println!("*z (MyBox) = {}", *z);
    println!();

    // Example 4: Deref coercion
    println!("Example 4: Deref Coercion");
    let name = MyBox::new(String::from("Rust"));
    // &MyBox<String> -> &String -> &str
    hello(&name);
    println!();

    // Example 5: Drop trait
    println!("Example 5: Drop Trait");
    {
        let _c = CustomSmartPointer {
            data: String::from("first"),
        };
        let _d = CustomSmartPointer {
            data: String::from("second"),
        };
        println!("CustomSmartPointers created");
    }
    println!("CustomSmartPointers dropped");
    println!();

    // Example 6: Early drop with std::mem::drop
    println!("Example 6: Early Drop");
    let c = CustomSmartPointer {
        data: String::from("early drop"),
    };
    println!("Created");
    drop(c); // Force early drop
    println!("Dropped early, before end of scope");
    println!();

    // Example 7: Rc<T> - Reference Counted
    println!("Example 7: Rc<T>");
    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    println!("Count after creating a: {}", Rc::strong_count(&a));

    let _b = Rc::clone(&a);
    println!("Count after creating b: {}", Rc::strong_count(&a));

    {
        let _c = Rc::clone(&a);
        println!("Count after creating c: {}", Rc::strong_count(&a));
    }

    println!("Count after c goes out of scope: {}", Rc::strong_count(&a));
    println!();

    // Example 8: RefCell<T> - Interior Mutability
    println!("Example 8: RefCell<T>");
    let data = RefCell::new(5);
    println!("Initial value: {}", data.borrow());

    *data.borrow_mut() += 10;
    println!("After modification: {}", data.borrow());

    // Multiple immutable borrows are OK
    let r1 = data.borrow();
    let r2 = data.borrow();
    println!("Two borrows: {} and {}", r1, r2);
    drop(r1);
    drop(r2);
    println!();

    // Example 9: Rc<RefCell<T>> - Shared mutable data
    println!("Example 9: Rc<RefCell<T>>");
    let value = Rc::new(RefCell::new(5));

    let a = Rc::clone(&value);
    let b = Rc::clone(&value);

    println!("Initial: {}", value.borrow());
    *a.borrow_mut() += 10;
    println!("After a modifies: {}", value.borrow());
    *b.borrow_mut() += 20;
    println!("After b modifies: {}", value.borrow());
    println!();

    // Example 10: Weak<T> - Avoiding reference cycles
    println!("Example 10: Weak<T>");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "Leaf strong count: {}, weak count: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "Branch strong count: {}, weak count: {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "Leaf strong count: {}, weak count: {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("After branch dropped:");
    println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!("Leaf strong count: {}", Rc::strong_count(&leaf));
    println!();

    println!("=== End of Day 18 Examples ===");
}

// Recursive type - Cons List
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn print_list(list: &List) {
    match list {
        Cons(val, next) => {
            print!("{} -> ", val);
            print_list(next);
        }
        Nil => println!("Nil"),
    }
}

// Custom smart pointer implementing Deref
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Custom smart pointer implementing Drop
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: '{}'", self.data);
    }
}

// Rc version of List
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// Tree structure with parent references using Weak
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

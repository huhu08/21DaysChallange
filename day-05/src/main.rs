// Day 5: Borrowing and References
// This file contains examples for Day 5 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 5: Borrowing and References ===\n");

    // Example 1: References
    println!("Example 1: References (Borrowing)");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
    println!();

    // Example 2: Multiple immutable references
    println!("Example 2: Multiple Immutable References");
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
    println!();

    // Example 3: Mutable references
    println!("Example 3: Mutable References");
    let mut s = String::from("hello");
    println!("Before: {}", s);
    change(&mut s);
    println!("After: {}", s);
    println!();

    // Example 4: One mutable reference at a time
    println!("Example 4: One Mutable Reference Rule");
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(" world");
        println!("r1: {}", r1);
    }
    let r2 = &mut s;
    r2.push_str("!");
    println!("r2: {}", r2);
    println!();

    // Example 5: Mixing references (NLL)
    println!("Example 5: Non-Lexical Lifetimes");
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 no longer used after this point
    let r3 = &mut s;
    r3.push_str(" world");
    println!("r3: {}", r3);
    println!();

    // Example 6: References in functions
    println!("Example 6: References in Functions");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("First word: {}", word);
    println!();

    // Example 7: Dereferencing
    println!("Example 7: Dereferencing");
    let x = 5;
    let y = &x;
    println!("x: {}", x);
    println!("*y: {}", *y);
    println!();

    // Example 8: Mutable reference modification
    println!("Example 8: Modifying Through Mutable Reference");
    let mut num = 10;
    let num_ref = &mut num;
    *num_ref += 5;
    println!("num after modification: {}", num);
    println!();

    // Example 9: References with structs
    println!("Example 9: References with Structs");
    let mut user = User {
        name: String::from("Alice"),
        age: 30,
    };
    print_user(&user);
    birthday(&mut user);
    print_user(&user);
    println!();

    println!("=== End of Day 5 Examples ===");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

struct User {
    name: String,
    age: u32,
}

fn print_user(user: &User) {
    println!("User: {}, Age: {}", user.name, user.age);
}

fn birthday(user: &mut User) {
    user.age += 1;
    println!("Happy Birthday, {}!", user.name);
}

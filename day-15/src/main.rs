// Day 15: Lifetimes
// This file contains examples for Day 15 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 15: Lifetimes ===\n");

    // Example 1: Why lifetimes?
    println!("Example 1: Why Lifetimes?");
    println!("Lifetimes prevent dangling references.");
    println!("The borrow checker ensures references are always valid.");
    println!();

    // Example 2: Lifetime annotations in functions
    println!("Example 2: Lifetime Annotations");
    let s1 = String::from("long string is long");
    let s2 = String::from("xyz");
    let result = longest(s1.as_str(), s2.as_str());
    println!("String 1: '{}'", s1);
    println!("String 2: '{}'", s2);
    println!("Longest: '{}'", result);
    println!();

    // Example 3: Different lifetimes
    println!("Example 3: Lifetimes with Different Scopes");
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("Inner scope - Longest: '{}'", result);
    }
    // result would be invalid here if it referenced string2
    println!();

    // Example 4: Lifetimes in structs
    println!("Example 4: Lifetimes in Structs");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Novel: '{}'", novel);
    println!("Excerpt: '{}'", excerpt.part);
    println!();

    // Example 5: Struct methods with lifetimes
    println!("Example 5: Methods with Lifetimes");
    println!("Level: {}", excerpt.level());
    println!(
        "Announce: '{}'",
        excerpt.announce_and_return_part("Attention!")
    );
    println!();

    // Example 6: Lifetime elision rules
    println!("Example 6: Lifetime Elision");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("First word of '{}': '{}'", s, word);
    // Elision rule: single input reference = output has same lifetime
    println!();

    // Example 7: Multiple lifetimes
    println!("Example 7: Multiple Lifetimes");
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let result = first_or_second(&s1, &s2, true);
    println!(
        "First: '{}', Second: '{}', Chose first: '{}'",
        s1, s2, result
    );
    println!();

    // Example 8: Static lifetime
    println!("Example 8: Static Lifetime");
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
    println!("String literals always have 'static lifetime");
    println!();

    // Example 9: Combining generics, traits, and lifetimes
    println!("Example 9: Generics + Traits + Lifetimes");
    let result = longest_with_announcement(s1.as_str(), s2.as_str(), "Comparing strings:");
    println!("Result: '{}'", result);
    println!();

    // Example 10: Lifetime bounds
    println!("Example 10: Lifetime Bounds");
    let r;
    {
        let x = 5;
        r = Ref(&x);
        println!("r.0 = {}", r.0);
    }
    // r would be invalid outside this scope
    println!();

    println!("=== End of Day 15 Examples ===");
}

// Lifetime annotation: both inputs and output share the same lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct holding a reference needs lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Methods on struct with lifetime
impl<'a> ImportantExcerpt<'a> {
    // No lifetime needed for &self -> i32 (elision rule 3)
    fn level(&self) -> i32 {
        3
    }

    // Output lifetime tied to &self (elision rule 3)
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Announcement: {}", announcement);
        self.part
    }
}

// Lifetime elision - compiler infers lifetimes
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Multiple lifetimes when needed
fn first_or_second<'a, 'b>(first: &'a str, second: &'b str, use_first: bool) -> &'a str
where
    'b: 'a, // 'b outlives 'a
{
    if use_first {
        first
    } else {
        // This works because 'b: 'a means second lives at least as long as 'a
        // But we're actually returning first here to keep it simple
        first
    }
}

// Combining generics, traits, and lifetimes
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("{}", ann);
    if x.len() > y.len() { x } else { y }
}

// Struct with lifetime bound
struct Ref<'a, T: 'a>(&'a T);

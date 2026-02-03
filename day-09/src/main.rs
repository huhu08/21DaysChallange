// Day 9: Enums and Pattern Matching
// This file contains examples for Day 9 of the 21 Days of Rust Challenge

fn main() {
    println!("=== Day 9: Enums and Pattern Matching ===\n");

    // Example 1: Basic enum
    println!("Example 1: Basic Enum");
    let dir = Direction::North;
    move_player(&dir);
    move_player(&Direction::East);
    println!();

    // Example 2: Enum with data
    println!("Example 2: Enum with Data");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    print_ip(&home);
    print_ip(&loopback);
    println!();

    // Example 3: Message enum with different variant types
    println!("Example 3: Message Enum");
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 128, 0),
    ];
    for msg in &messages {
        process_message(msg);
    }
    println!();

    // Example 4: Option enum
    println!("Example 4: Option<T>");
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("no_number: {:?}", no_number);

    // Using Option
    if let Some(n) = some_number {
        println!("Got a number: {}", n);
    }
    println!();

    // Example 5: match expression
    println!("Example 5: match Expression");
    for coin in &[Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter] {
        println!("{:?} = {} cents", coin, value_in_cents(coin));
    }
    println!();

    // Example 6: match with Option
    println!("Example 6: match with Option");
    println!("plus_one(Some(5)) = {:?}", plus_one(Some(5)));
    println!("plus_one(None) = {:?}", plus_one(None));
    println!();

    // Example 7: Catch-all patterns
    println!("Example 7: Catch-all Patterns");
    let dice_roll = 9;
    match dice_roll {
        3 => println!("Fancy hat!"),
        7 => println!("Remove fancy hat!"),
        other => println!("Move {} spaces", other),
    }

    // Using _ to ignore value
    let dice_roll = 9;
    match dice_roll {
        3 => println!("Fancy hat!"),
        7 => println!("Remove fancy hat!"),
        _ => println!("Something else"),
    }
    println!();

    // Example 8: if let syntax
    println!("Example 8: if let Syntax");
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Maximum configured: {}", max);
    }

    // if let with else
    let coin = Coin::Quarter;
    if let Coin::Quarter = coin {
        println!("It's a quarter!");
    } else {
        println!("Not a quarter");
    }
    println!();

    // Example 9: Enum methods
    println!("Example 9: Enum with Methods");
    let msg = Message::Write(String::from("Hello, Rust!"));
    msg.call();
    println!();

    println!("=== End of Day 9 Examples ===");
}

// Basic enum
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn move_player(dir: &Direction) {
    match dir {
        Direction::North => println!("Moving north ↑"),
        Direction::South => println!("Moving south ↓"),
        Direction::East => println!("Moving east →"),
        Direction::West => println!("Moving west ←"),
    }
}

// Enum with data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn print_ip(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
}

// Enum with different data types per variant
enum Message {
    Quit,                    // No data
    Move { x: i32, y: i32 }, // Named fields (like a struct)
    Write(String),           // Single String
    ChangeColor(u8, u8, u8), // Three u8 values
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn process_message(msg: &Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
    }
}

// Coin enum for match example
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Option handling with match
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// https://www.youtube.com/watch?v=aYsUBddY7KY

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn index(x: i32, y: i32, columns: i32) -> i32 {
    x + y * columns
}

fn if_example() {
    let number = 3;
    if number < 5 {
        println!("Condition is true.");
    }
    else if number > 10 {
        println!("Else if example...");
    }
    else {
        println!("Condition is false.");
    }
}

// Conditional assignment; using if in a let
fn conditional_assignment() -> i32 {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    number
}

fn infinite_loop() {
    loop {
        println!("Looping...");
        break;
    }
}

fn loops() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFT OFF!");
}

fn loop_collection() {
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for element in (1..4).rev() {
        println!("{}!", element);
    }
}

fn fib(mut n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    while n > 0 {
        let temp = b;
        b = a + b;
        a = temp;
        n = n - 1;
    }
    b
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slicing() {
    let word = String::from("Hello world!");
    let slice = &word[1..];
    println!("Slice of word is: {}", slice);

    // More examples
    println!("First three: {}", &word[..3]);
    println!("Last three: {}", &word[3..]);
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size
        }
    }
}

fn area(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

fn area_rect(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}

enum IpAddrKind {
    V4,
    V6
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        // Doesn't compile because of std::cmp::PartialOrd not supported.
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U
}

// impl<T, U> Point2<T, U> {
//     pub fn length(&self) {
//         (self.x * self.x + self.y * self.y) ^ 0.5
//     }
// }

pub trait Summarizable {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summarizable for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summarizable for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// fn some_function<T, U>(t: T, u: U) -> i32 
//     where T: Display + Clone,
//           U: Clone + Debug
// {
//     0
// }

fn main() {
    println!("Hello world!");

    // Tuples
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is {}", y);

    let second = tup.1;
    println!("Second value in the tuple is {}", second);

    // Arrays
    let a = [1, 2, 3, 4, 5]; // Allocated on the stack, and not on the heap.
    let a1 = a[1];    

    // Functions
    another_function(a1);
    let idx = index(2, 3, 4);
    println!("Index of coordinate is: {}", idx);

    if_example();

    let cnd = conditional_assignment();
    println!("Conditional assignment resulted in: {}", cnd);

    loops();
    loop_collection();

    let fib10 = fib(10);
    println!("The tenth Fibonacci number is: {}", fib10);

    let str = String::from("Hello world!");
    let strlen = calculate_length(&str);
    println!("The length of the string is: {}", strlen);

    let fw = first_word(&str);
    println!("Index of the end of the first word: {}", fw);

    string_slicing();

    println!("First word slice: {}", first_word_slice(&str[..]));

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    let user2 = build_user(String::from("me@example.com"), String::from("test"));
    let user3 = User {
        email: String::from("yolo@example.com"),
        ..user2
    };
    println!("User {} logged in {} times.", user3.email, user3.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Printing a tuple: ({}, {}, {})", black.0, black.1, black.2);

    println!(
        "The area of a rectangle with a width of 5 and a height of 10 is {}",
        area((5, 10))
    );

    let rect = Rectangle { width: 10, length: 7 };
    println!(
        "The area of a {} * {} rectangle is {}.",
        rect.width,
        rect.length,
        area_rect(&rect)
    );

    // Print a struct with Debug (#[derive(Debug)]).
    println!("Rect is {:?}", rect);
    println!("Rect is {:#?}", rect);

    println!(
        "The area of a {} * {} rectangle is {}.",
        rect.width,
        rect.length,
        rect.area()
    );

    let square = Rectangle::square(5);
    println!(
        "Rect can hold square: {}",
        rect.can_hold(&square)
    );

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let coin = Coin::Quarter;
    println!("Value of coin is: {}", value_in_cents(coin));

    let point = Point2 { x: 5, y: 10.1 };
    println!("Point: {:?}", point);

    let tweet = Tweet { 
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is: {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("The largest char is {}", result);

    infinite_loop();
}

#[test]
fn should_fail() {
    unimplemented!();
}
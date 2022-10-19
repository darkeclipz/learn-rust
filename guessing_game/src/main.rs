use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MAX_POINTS: i32 = 100_000;


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn tuple() {
    let t = (500, 250, 10);
    let (x, y, z) = t;

    println!("X: {}, Y: {}, Z: {}", x, y, z);
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    let range = 1..10;
    let empty = [0; 100];
    println!("len(A)={}, len(range)={}, len(empty)={}", a.len(), range.len(), empty.len());
}

fn launch() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFT OFF!");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn test() {

    let mut n: u32 = 10;
    n = 11;

    test2(n);

    let n = 12;

    // n = 18;


    let text = String::from("Hello world");
    let example: &str = "Hello world";

    test3(example);
    test4(example);

    println!("{}", example);
}

fn test2(mut n: u32) {
    n = 12;
    println!("{}", n);
}

fn test3(mut text: &str) -> &str {
    text = "Hello";

    if 12 > 10 {
        return text;
    }


    
        
    text
}

fn test4(mut text: &str) {
    text = "Edit the string!";
}

use std::fs;

fn read_file(file_path: &str) {
    println!("Reading file with path: {file_path}");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file.");
    println!("With text:\n{contents}");
}

fn main() {

    let file = String::from("example.txt");
    read_file(&file);

    // test();

    // // Basic stuff
    // tuple();
    // array();
    // launch();
    // let sentence = String::from("first word!");
    // println!("First word is '{}'.", first_word(&sentence));

    // // Structures
    // let user = build_user(
    //     String::from("hello@optimalalgoritm.com"), 
    //     String::from("Optimal Algorithm")
    // );

    // println!(
    //     "Username '{}' logged in {} times.", 
    //     user.username, 
    //     user.sign_in_count
    // );

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);

    // let rect1 = Rectangle { width: 30, height: 50 };
    // println!("Area of {:?} is {}", rect1, rect1.area());

    // let rect2 = Rectangle { width: 25, height: 60 };
    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // let square = Rectangle::square(25);
    // println!("Square {:?}", square);


    // // Guessing game
    // println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1..101);

    // loop {
    //     println!("Please input your guess.");

    //     let mut guess = String::new();
    
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read the line.");
    
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    
    //     println!("You guessed: {}", guess);
        
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    // println!("Max points is {}.", MAX_POINTS);
}

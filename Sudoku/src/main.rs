use std::io::stdin;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn read_board() -> String {
    let mut buffer = String::new();
    println!("Enter a sudoku string:");
    stdin().read_line(&mut buffer).expect("Error reading input.");
    if buffer.len() != 81 {
        panic!("Input string must be 81 characters, got '{}' instead.", buffer);
    }
    buffer
}

fn main() {
    let board_string = read_board();
    println!("Given board string: {}", board_string);
}

#[test]
fn test_read_board() {
    unimplemented!();
}


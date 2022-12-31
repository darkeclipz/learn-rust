use std::io::stdin;

// 070040800009005060060000003004100070000200006020034000100008090000600401007000000

// fn read_board() -> Vec<u32> {
//     let mut buffer = String::new();
//     println!("Enter a sudoku string:");
//     stdin().read_line(&mut buffer).expect("Error reading input.");
//     if buffer.len() != 81 {
//         panic!("Input string must be 81 characters, got '{}' instead.", buffer);
//     }
//     string_to_board(&buffer)
// }

fn string_to_board(puzzle_string: &String) -> Vec< u32> {
    let mut board = Vec::new();
    for char in puzzle_string.trim().chars() {
        let cell = (char as u32) - ('0' as u32);
        board.push(cell);
    }
    board
}

fn print_board(board: &Vec<u32>) {
    println!("{:?}", board);
}

fn solve(k: usize, board: Vec<u32>) -> bool {

    if k >= 81 {
        return true
    }

    if board[k] > 0 {
        return solve(k + 1, board);
    }

    for n in 1..10 {
        board[k] = n;

        if solve(k + 1, board) {
            return true;
        }
        
        board[k] = 0;
        
    }

    false
}

fn main() {
    let puzzle_string = String::from(
        "070040800009005060060000003004100070000200006020034000100008090000600401007000000");
    let mut board = string_to_board(&puzzle_string);
    let result = solve(0, board);


    let mut domains = vec![vec![true; 10]; 81];

    for i in 0..81 {
        domains[i][board[i] as usize] = false;
    }
    
    print_board(&board);
}

#[test]
fn test_read_board() {
    unimplemented!();
}


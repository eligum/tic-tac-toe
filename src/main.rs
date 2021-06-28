use rand::Rng;
use std::io::{self, Write}; // The Rng trait defines methods that random number generators implement

fn print_game_menu() {
    println!("1) Play a game of tic-tac-toe.");
    println!("2) Configure settings.");
    println!("3) Select saved game.");
    println!("4) Exit.");
}

/// Read an integer from stdin that it is between a certain range.
///
/// # Parameters
///
/// The parameter `message` is the message to be displayed before user input, if `None` is encountered
/// a default one will be provided.
fn read_input_range(min: i32, max: i32, message: Option<&str>) -> i32 {
    loop {
        match message {
            Some(msg) => print!("{}", msg),
            None => print!("Enter the number of the option you want to select: "),
        };
        io::stdout().flush().expect("Failed to flush");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read input");

        match answer.trim().parse::<i32>() {
            Ok(op) => {
                if op < min || op > max {
                    println!(
                        "Please enter a valid number, possible values range from {} to {}.",
                        min, max
                    );
                    continue;
                } else {
                    break op;
                }
            }
            Err(_) => {
                println!(
                    "Please enter a valid number, possible values range from {} to {}.",
                    min, max
                );
                continue;
            }
        };
    }
}

// 3x3 board;
// -1 represents an empty cell
//  0 represents the Os
//  1 represents the Xs
struct Board {
    cells: [i8; 9],
}

impl Board {
    fn print_board(&self) {
        let mut fmt_board = ['e'; 9];

        for i in 0..9 {
            fmt_board[i] = match self.cells[i] {
                0 => 'X',
                1 => '$',
                _ => (b'1' + i as u8) as char,
            };
        }
        for i in 0..3 {
            println!(
                " {} | {} | {} ",
                fmt_board[3 * i],
                fmt_board[3 * i + 1],
                fmt_board[3 * i + 2]
            );
            if i < 2 {
                println!("-----------");
            }
        }
    }

    fn set_cell(&mut self, index: usize, value: i8) {
        self.cells[index] = value; // self.cells[i * 3 + j] = value;
    }

    fn is_empty_cell(&self, index: usize) -> bool {
        self.cells[index] != 0 && self.cells[index] != 1
    }

    fn is_winning_position(&self) -> bool {
        // Check horizontal
        for i in 0..3 {
            if self.cells[i * 3] == self.cells[i * 3 + 1]
                && self.cells[i * 3 + 1] == self.cells[i * 3 + 2]
                && (self.cells[i * 3] == 0 || self.cells[i * 3] == 1)
            {
                return true;
            }
        }
        // Check vertical
        for i in 0..3 {
            if self.cells[i] == self.cells[i + 3]
                && self.cells[i + 3] == self.cells[i + 6]
                && (self.cells[i] == 0 || self.cells[i] == 1)
            {
                return true;
            }
        }
        // Check diagonals
        if ((self.cells[0] == self.cells[4] && self.cells[4] == self.cells[8])
            || (self.cells[2] == self.cells[4] && self.cells[4] == self.cells[6]))
            && (self.cells[5] == 0 || self.cells[5] == 1)
        {
            return true;
        }

        false
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..10);
    let mut brd = Board { cells: [2; 9] };
    let mut ans: i32;

    print_game_menu();
    ans = read_input_range(1, 4, None);

    match ans {
        1 => {
            let mut turn: i8 = 0;
            clear_screen();

            // Main game loop
            let mut count = 0;
            loop {
                brd.print_board();
                if turn == 1 {
                    turn = 2;
                } else {
                    turn = 1;
                }
                println!("\nPLAYER {}", turn);
                loop {
                    ans = read_input_range(
                        1,
                        9,
                        Some("Select where do you want to play your next move: "),
                    );
                    if brd.is_empty_cell((ans - 1) as usize) {
                        break;
                    } else {
                        println!("Illegal move! Try again please.");
                    }
                }
                brd.set_cell((ans - 1) as usize, turn - 1);

                clear_screen();

                if brd.is_winning_position() {
                    brd.print_board();
                    println!("\nPLAYER {} won the game!", turn);
                    break;
                } else if count == 8 {
                    brd.print_board();
                    println!("\nIt's a draw.");
                    break;
                }
                count += 1;
            }
        }
        4 => {
            println!("Thanks for playing!");
        }
        _ => {
            println!("Not implemented yet");
        }
    }
}

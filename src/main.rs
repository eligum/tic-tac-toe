use rand::Rng;
use std::io::{self, Write}; // The Rng trait defines methods that random number generators implement

// Damage formula: dmg = (pierce * base_damage) + ((1 - pierce) * base_damage * (ATCK or 100 / (ATCK or 100 + DEF))

fn print_game_menu() {
    println!("1) Play a game of tic-tac-toe.");
    println!("2) Configure settings.");
    println!("3) Select saved game.");
    println!("4) Exit.");
}

fn read_input_range(min: i32, max: i32) -> i32 {
    loop {
        print!("Enter the number of the option you want to select: ");
        io::stdout().flush().expect("Failed to flush");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read input");

        match answer.trim().parse::<i32>() {
            Ok(op) => {
                if op < min || op > max {
                    println!("Please enter a valid option, possible values range from {} to {}.", min, max);
                    continue;
                } else {
                    break op;
                }
            }
            Err(_) => {
                println!("Please enter a valid option, possible values range from {} to {}.", min, max);
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
                0 => 'O',
                1 => 'X',
                _ => ('1' as u8 + i as u8) as char,
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

    fn check_for_winning_position(&self) -> bool {
        // Check horizontal
        for i in 0..3 {
            if self.cells[i * 3] == self.cells[i * 3 + 1]
                && self.cells[i * 3 + 1] == self.cells[i * 3 + 2]
            {
                return true;
            }
        }
        // Check vertical
        for i in 0..3 {
            if self.cells[i] == self.cells[i + 3] && self.cells[i + 3] == self.cells[i + 6] {
                return true;
            }
        }
        // Check diagonals
        if (self.cells[0] == self.cells[4] && self.cells[4] == self.cells[8])
            || (self.cells[2] == self.cells[4] && self.cells[4] == self.cells[6])
        {
            return true;
        }

        false
    }
}

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..10);
    let brd = Board { cells: [-1; 9] };

    print_game_menu();
    brd.print_board();

    read_input_range(1, 4);
}

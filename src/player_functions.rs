use std::io;
use rand::Rng;

use crate::board::Board;

pub fn ask_num_players() -> i32 {
    loop {
        println!("How many players are going to play?");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) if num >= 1 && num <= 2 => return num,
            Ok(_) => println!("Please enter a number between 1 and 2"),
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

pub fn two_players(board: &Board) -> i32 {
    let mut players_turn = 1;
    let mut player_won = 0;
    while player_won == 0{
        for (i, row) in board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                print!("{}{}", if cell == 0 { " " } else if cell == 1 { "O" } else { "X" }, if j < 2 { "|" } else { "" });
            }
            println!();
            if i < 2 {
                println!("------");
            }
        }
    }
    player_won
}






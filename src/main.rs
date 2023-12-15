mod board;
mod player_functions;

use crate::board::Board;
use player_functions::{ask_num_players, two_players};


fn main() {
    let filas = 3;
    let columnas = 3;
    let board = Board::new(filas, columnas);
    let num_players = ask_num_players();

    play(&num_players, &board)
}

fn play(num_players: &i32, board: &Board) {
    let player_won: i32;
    if num_players.to_owned() == 1 {
        player_won = two_players(&board);
    } else {
        player_won = two_players(&board);
    }
    println!("Congratulations! Player {} won!", player_won);
}

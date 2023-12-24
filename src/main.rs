// En main.rs (o lib.rs)
mod board;
mod player_functions;

use crate::board::Board;
use player_functions::{two_players, ask_num_players, single_player};

fn main() {
    let filas = 3;
    let columnas = 3;
    let mut board = Board::new(filas, columnas);
    let num_players = ask_num_players();

    play(&num_players, &mut board);
}


fn play(num_players: &i32, board: &mut Board) {
    if num_players.to_owned() == 1 {
        single_player(board);
    } else {
        two_players(board);
    }
}
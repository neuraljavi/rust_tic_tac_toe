mod board_functions;
mod player_functions;

use board_functions::create_board;
use player_functions::ask_num_players;


fn main() {
    let filas = 3;
    let columnas = 3;
    let board = create_board(filas, columnas);
    let num_players = ask_num_players();

    play(&num_players, &board)

}

fn play(num_players: &i32, board: &Vec<Vec<i32>>) {
    
}

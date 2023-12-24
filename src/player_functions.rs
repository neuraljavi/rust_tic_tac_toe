use std::io;

use crate::Board;

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

fn read_player_input(board: &Board) -> (i32, i32) {
    let mut input = String::new();
    let mut final_input: (i32, i32) = (0, 0);
    let mut correct_input = false;
    while !correct_input {
        println!("Enter the row number");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let row: i32 = input.trim().parse().expect("Please enter a valid number");
        println!("Enter the column number");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let column: i32 = input.trim().parse().expect("Please enter a valid number");
        if row >= 1 && row <= 3 && column >= 1 && column <= 3 {
            final_input = (row, column);
            correct_input = true;
        } else {
            println!("Please enter a valid number");
        }
        if board.check_occupied(row, column) {
            println!("Square already occupied");
            correct_input = false;
        }
    }
    final_input
}

pub fn two_players(board: &mut Board) {
    let mut players_turn = 1;
    let mut player_won = 0;
    let mut moves = 0;
    while player_won == 0 && moves < board.get_num_rows() * board.get_num_columns() {
        board.print();
        println!("Player {}'s turn", players_turn);
        let player_input = read_player_input(board);
        board.insert(
            players_turn,
            player_input.0,
            player_input.1,
        );
        if check_winner(board) {
            player_won = players_turn;
        }
        moves += 1;
        if players_turn == 1 {
            players_turn = 2;
        } else {
            players_turn = 1;
        }
    }
    if player_won == 0 {
        println!("It's a tie!")
    } else {
        println!("Player {} won!", player_won);
    }
}

pub fn single_player(board: &mut Board) {
    let mut players_turn = 1;
    let mut player_won = 0;
    let mut moves = 0;
    while player_won == 0 && moves < board.get_num_rows() * board.get_num_columns() {
        board.print();
        println!("Player {}'s turn", players_turn);
        if players_turn == 1 {
            let player_input = read_player_input(board);
            board.insert(
                players_turn,
                player_input.0,
                player_input.1,
            );
            if check_winner(board) {
                player_won = players_turn;
            }
            moves += 1;
            if players_turn == 1 {
                players_turn = 2;
            } else {
                players_turn = 1;
            }
        } else {
            make_move(board, players_turn)
        }
    }
    if player_won == 0 {
        println!("It's a tie!")
    } else {
        println!("Player {} won!", player_won);
    }
}

fn check_winner(board: &Board) -> bool {
    for i in 0..board.get_num_rows() {
        if board.obtain_value(i, 0) == board.obtain_value(i, 1)
            && board.obtain_value(i, 1) == board.obtain_value(i, 2)
        {
            return true;
        }
        if board.obtain_value(0, i) == board.obtain_value(1, i)
            && board.obtain_value(1, i) == board.obtain_value(2, i)
        {
            return true;
        }
    }
    if board.obtain_value(0, 0) == board.obtain_value(1, 1)
        && board.obtain_value(1, 1) == board.obtain_value(2, 2)
    {
        return true;
    } else if board.obtain_value(0, 2) == board.obtain_value(1, 1)
        && board.obtain_value(1, 1) == board.obtain_value(2, 0)
    {
        return true;
    }
    false
}

// Función para que la IA realice su movimiento
fn make_move(board: &mut Board, player: i32) {
    // Recorrer el tablero para encontrar la primera casilla disponible
    for i in 0..board.get_num_rows() {
        for j in 0..board.get_num_columns() {
            if !board.check_occupied(i, j) {
                // Casilla disponible, colocar la ficha del jugador
                board.insert(player, i, j);
                return; // Terminar después de realizar el movimiento
            }
        }
    }
}

pub fn create_board(rows: i32, columns: i32) -> Vec<Vec<i32>> {
    let mut board = Vec::new();
    for _ in 0..rows {
        let mut row = Vec::new();
        for _ in 0..columns {
            row.push(0);
        }
        board.push(row);
    }
    board
}
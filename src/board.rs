pub struct Board {
    columns: i32,
    rows: i32,
    board: Vec<Vec<i32>>,
}

impl Board {
    pub fn new(columns: i32, rows: i32) -> Self {
        let mut board = Vec::new();
        for _ in 0..rows {
            let row = vec![0; columns as usize];
            board.push(row);
        }

        Board {
            columns,
            rows,
            board,
        }
    }

    pub fn print(&self) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let symbol = match cell {
                    0 => " ",
                    1 => "x",
                    2 => "o",
                    _ => panic!("Valor no esperado en el tablero"),
                };
    
                print!("{}{}", symbol, if j < self.columns as usize - 1 { "|" } else { "" });
            }
            println!();
    
            if i < self.rows as usize - 1 {
                // Línea separadora entre filas
                println!("{}", "-".repeat((self.columns * 2 - 1) as usize));
            }
        }
    }

    pub fn insert(&mut self, player: i32, row: usize, column: usize) {
        if row < self.rows as usize && column < self.columns as usize {
            self.board[row][column] = player;
        }
    }

    pub fn get_num_rows(&self) -> i32 {
        self.rows
    }

    pub fn get_board(&self) -> Vec<Vec<i32>> {
        self.board
    }
}
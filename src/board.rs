pub struct Board {
    columns: i32,
    rows: i32,
    board: Vec<Vec<i32>>,
}

impl Board {
    pub fn new(columns: i32, rows: i32) -> Self {
        let board = (0..rows)
            .map(|_| vec![0; columns as usize])
            .collect();
    
        Board { columns, rows, board }
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

    pub fn insert(&mut self, player: i32, row: i32, column: i32) {
        if row < self.rows && column < self.columns {
            self.board[row as usize][column as usize] = player;
        }
    }

    pub fn get_num_rows(&self) -> i32 {
        self.rows
    }

    pub fn get_num_columns(&self) -> i32 {
        self.columns
    }

    pub fn obtain_value(&self, row: i32, column: i32) -> Option<i32> {
        if row < self.board.len() as i32 && column < self.board[row as usize].len() as i32 {
            Some(self.board[row as usize][column as usize])
        } else {
            None
        }
    }

    pub fn check_occupied(&self, row: i32, column: i32) -> bool {
        if self.board[row as usize][column as usize] == 0 {
            false
        } else {
            true
        }
    }
}
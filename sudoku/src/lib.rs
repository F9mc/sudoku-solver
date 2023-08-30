pub mod sudoku {
    use std::fmt;
    extern crate csv;
    use csv::Reader;
    pub struct Sudoku {
        // Raws of Columns
        matrix: Vec<Vec<Cell>>,
        size: usize,
    }
    #[derive(Clone)]
    #[warn(dead_code)]
    pub enum Cell {
        FinalValue(usize),
        PossibleValues(Vec<usize>),
        UnknownState,
    }

    impl Sudoku {
        pub fn new() -> Sudoku {
            let base_cell: Cell = Cell::UnknownState;
            let len: usize = 9;
            let matrix: Vec<Vec<Cell>> = vec![vec![base_cell; len]; len];
            Sudoku {
                matrix: matrix,
                size: len,
            }
        }

        pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
            if x < self.size && y < self.size {
                Some(&self.matrix[x][y])
            } else {
                None
            }
        }

        pub fn clone_board(&self) -> Sudoku {
            let mut new_matrix = Vec::with_capacity(self.size);
            for row in &self.matrix {
                new_matrix.push(row.clone());
            }
            Sudoku {
                matrix: new_matrix,
                size: self.size,
            }
        }

        pub fn set_cell(&mut self, x: usize, y: usize, value: Cell) -> Result<(), String> {
            if x < self.size && y < self.size {
                self.matrix[x][y] = value;
                Ok(())
            } else {
                Err("The arguments x and y are not correct".to_string())
            }
        }

        pub fn read(file_path: String) -> Result<Sudoku, String> {
            let mut board = Sudoku::new();

            Ok(board)
        }

        fn format_matrix(&self) -> String {
            let mut result = String::new();
            for i in 0..self.size {
                if i > 0 && i % 3 == 0 {
                    // Add a horizontal line after every 3 rows
                    result.push_str("+-----+-----+-----+\n");
                }
                for j in 0..self.size {
                    if j > 0 && j % 3 == 0 {
                        // Add a vertical line after every 3 columns
                        result.push('|');
                    }
                    match self.get_cell(i, j) {
                        Some(Cell::FinalValue(value)) => result.push_str(&format!("{:?} ", value)),
                        _ => result.push_str(&format!("? ",)),
                    }
                }
                result.push('\n');
            }
            result.push_str("+-----+-----+-----+\n");
            result
        }
    }

    impl fmt::Display for Sudoku {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.format_matrix())
        }
    }
}

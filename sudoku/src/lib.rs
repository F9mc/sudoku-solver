pub mod sudoku {
    use csv;
    use serde::Deserialize;
    use std::{fmt, vec};

    #[derive(Debug, Deserialize)]
    struct CsvRecords {
        x: usize,
        y: usize,
        value: usize,
    }
    pub struct Sudoku {
        // Raws of Columns
        matrix: Vec<Vec<Cell>>,
        size: usize,
    }
    #[derive(Clone, Debug)]
    #[warn(dead_code)]
    pub enum Cell {
        FinalValue(usize),
        PossibleValues(Vec<usize>),
        UnknownState,
    }

    impl Cell {
        pub fn remove_value(&self, v: usize) -> Result<Cell, String> {
            match &self {
                Cell::PossibleValues(values) => {
                    values.clone().retain(|&x| x != v);
                    if values.len() == 1 {
                        Ok(Cell::FinalValue(values[0]))
                    } else {
                        Ok(Cell::PossibleValues(values.clone()))
                    }
                }
                _ => Err("Impossible to remove value from this cell".to_string()),
            }
        }
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
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .delimiter(b',')
                .flexible(true)
                .double_quote(false)
                .from_path(file_path)
                .unwrap();
            for result in rdr.deserialize() {
                let record: CsvRecords = result.unwrap();
                board
                    .set_cell(record.x, record.y, Cell::FinalValue(record.value))
                    .unwrap();
            }
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
                        Some(Cell::PossibleValues(_)) => result.push_str(&format!("* ")),
                        _ => result.push_str(&format!("? ",)),
                    }
                }
                result.push('\n');
            }
            result.push_str("+-----+-----+-----+\n");
            result
        }

        pub fn get_row(&self, row: usize) -> Result<Vec<usize>, String> {
            if row < self.size {
                let mut result: Vec<usize> = Vec::new();
                for i in 0..self.size {
                    match self.get_cell(row, i) {
                        Some(Cell::FinalValue(v)) => result.push(*v),
                        Some(Cell::PossibleValues(values)) => {
                            for v in values.iter() {
                                result.push(*v)
                            }
                        }
                        _ => print!("Value {i}-{row} not found"),
                    }
                }
                result.sort();
                result.dedup();
                Ok(result)
            } else {
                Err("Out of bound".to_string())
            }
        }

        pub fn get_col(&self, col: usize) -> Result<Vec<usize>, String> {
            if col < self.size {
                let mut result: Vec<usize> = Vec::new();
                for i in 0..self.size {
                    match self.get_cell(i, col) {
                        Some(Cell::FinalValue(v)) => result.push(*v),
                        Some(Cell::PossibleValues(values)) => {
                            for v in values.iter() {
                                result.push(*v)
                            }
                        }
                        _ => print!("Value {i}-{col} not found"),
                    }
                }
                result.sort();
                result.dedup();
                Ok(result)
            } else {
                Err("Out of bound".to_string())
            }
        }

        pub fn get_region(&self, x: usize, y: usize) -> Result<Vec<usize>, String> {
            let x_start = (x / 3) * 3;
            let y_start = (y / 3) * 3;
            if x_start < self.size && y_start < self.size {
                let mut result: Vec<usize> = vec![];
                for i in x_start..x_start + 3 {
                    for j in y_start..y_start + 3 {
                        match self.get_cell(i, j) {
                            Some(Cell::FinalValue(v)) => result.push(*v),
                            Some(Cell::PossibleValues(values)) => {
                                for v in values.iter() {
                                    result.push(*v)
                                }
                            }
                            _ => (),
                        }
                    }
                }
                result.sort();
                result.dedup();
                Ok(result)
            } else {
                Err("Out of bound".to_string())
            }
        }

        pub fn solve(&self) -> Result<Sudoku, String> {
            todo!()
        }
    }

    impl fmt::Display for Sudoku {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.format_matrix())
        }
    }
}

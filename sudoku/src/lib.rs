pub mod sudoku {
    use csv;
    use serde::Deserialize;
    use std::{fmt, result, vec};

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

        pub fn set_cell(&mut self, x: usize, y: usize, values: Vec<usize>) -> Result<(), String> {
            if x < self.size && y < self.size {
                match values.len() {
                    0 => return Err("No values where specified".to_string()),
                    1 => {
                        println!("Final value {:} found in {x}-{y} ", values[0]);
                        self.matrix[x][y] = Cell::FinalValue(values[0])
                    }
                    _ => {
                        println!("Possibles values {:?} set in {x}-{y}", values);
                        self.matrix[x][y] = Cell::PossibleValues(values)
                    }
                }
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
                    .set_cell(record.x, record.y, vec![record.value])
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
                        _ => (),
                    }
                }
                result.sort();
                result.dedup();
                println!("Value of row {row} found: {:?}", result);
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
                        _ => (),
                    }
                }
                result.sort();
                result.dedup();
                println!("Value of col {col} found: {:?}", result);
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
                            _ => (),
                        }
                    }
                }
                result.sort();
                result.dedup();
                println!("Value of region found: {:?}", result);
                Ok(result)
            } else {
                Err("Out of bound".to_string())
            }
        }

        pub fn solve(&mut self) -> Result<(), String> {
            println!("Setp 0: \n{:#}", self);

            // Initialize
            for l in 0..100 {
                for i in 0..self.size {
                    for j in 0..self.size {
                        match self.get_cell(i, j) {
                            Some(Cell::FinalValue(_)) => println!("Value {i} - {j} already set"),
                            None => println!("Error getting value {i}-{j}"),
                            _ => {
                                let mut values = Vec::new();
                                values.extend(self.get_col(j).unwrap());
                                values.extend(self.get_row(i).unwrap());
                                values.extend(self.get_region(i, j).unwrap());
                                values.dedup();
                                println!("Found values {:?} already used", values);
                                let mut possibles_values: Vec<usize> = Vec::new();
                                for k in 1..self.size + 1 {
                                    if !values.contains(&k) {
                                        possibles_values.push(k)
                                    }
                                }
                                self.set_cell(i, j, possibles_values).unwrap();
                            }
                        }
                    }
                }
                if self.is_solved() {
                    println!("Solved in {l} steps\nSolution:\n{:#}", self);
                    break;
                } else {
                    println!("After step {l}:\n{:#}", self);
                }
            }
            Ok(())
        }

        pub fn is_solved(&self) -> bool {
            for i in 0..self.size {
                for j in 0..self.size {
                    match self.get_cell(i, j) {
                        Some(Cell::PossibleValues(_)) => return false,
                        Some(Cell::UnknownState) => return false,
                        _ => (),
                    }
                }
            }
            true
        }
    }

    impl fmt::Display for Sudoku {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.format_matrix())
        }
    }
}

use ::sudoku::sudoku::Cell;
use sudoku::sudoku;

fn main() {
    let mut board: sudoku::Sudoku = sudoku::Sudoku::new();
    board.set_cell(0, 0, Cell::FinalValue(0)).unwrap();
    print!("{board}")
}

#[cfg(test)]
mod tests {
    use sudoku::sudoku::{Cell, Sudoku};

    #[test]
    fn test_add() {
        let x: usize = 9;
        let mut board = Sudoku::new();

        board.set_cell(1, 2, Cell::FinalValue(x)).unwrap();
        match board.get_cell(1, 2) {
            Some(Cell::FinalValue(v)) => assert_eq!(v, &x),
            _ => panic!("Wrong value"),
        }
    }
}

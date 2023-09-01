use ::sudoku::sudoku::Cell;
use sudoku::sudoku::Sudoku;

fn main() {
    let board: Sudoku = sudoku::sudoku::Sudoku::read("tests/real-board.csv".to_string()).unwrap();
    print!("{:#}", board)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use sudoku::sudoku::{Cell, Sudoku};

    #[test]
    fn test_update_cell() {
        let x: usize = 9;
        let mut board = Sudoku::new();

        board.set_cell(1, 2, Cell::FinalValue(x)).unwrap();
        match board.get_cell(1, 2) {
            Some(Cell::FinalValue(v)) => assert_eq!(v, &x),
            _ => panic!("Wrong value"),
        }
    }

    #[test]
    fn test_reader() {
        let board: Sudoku = sudoku::sudoku::Sudoku::read("tests/board.csv".to_string()).unwrap();
        match board.get_cell(1, 1) {
            Some(Cell::FinalValue(v)) => assert_eq!(v, &3),
            _ => panic!("Wrong value"),
        }

        match board.get_cell(1, 2) {
            Some(Cell::FinalValue(v)) => assert_eq!(v, &4),
            _ => panic!("Wrong value"),
        }

        match board.get_cell(5, 6) {
            Some(Cell::FinalValue(v)) => assert_eq!(v, &7),
            _ => panic!("Wrong value"),
        }
    }

    #[test]
    fn test_setters_and_getters() {
        let mut board = Sudoku::new();
        board
            .set_cell(0, 0, Cell::PossibleValues(vec![1, 4, 7]))
            .unwrap();
        board
            .set_cell(0, 7, Cell::PossibleValues(vec![2, 4, 7, 8, 9]))
            .unwrap();
        board.set_cell(1, 0, Cell::FinalValue(2)).unwrap();
        board.set_cell(2, 0, Cell::FinalValue(3)).unwrap();
        board.set_cell(3, 0, Cell::FinalValue(5)).unwrap();
        board.set_cell(4, 0, Cell::FinalValue(6)).unwrap();
        board.set_cell(5, 0, Cell::FinalValue(7)).unwrap();
        board
            .set_cell(6, 0, Cell::PossibleValues(vec![8, 9]))
            .unwrap();

        assert_eq!(
            board.set_cell(19, 1, Cell::FinalValue(0)),
            Err("The arguments x and y are not correct".to_string())
        );

        match board.get_cell(1, 0) {
            Some(Cell::FinalValue(v)) => assert_eq!(v, &2),
            _ => panic!("Wrong value"),
        }

        match board.get_cell(100, 0) {
            Some(_) => panic!("Should not have return something"),
            _ => print!("Worked"),
        }

        assert_eq!(board.get_col(0), Ok(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert_eq!(board.get_row(0), Ok(vec![1, 2, 4, 7, 8, 9]));
        assert_eq!(board.get_region(0, 1), Ok(vec![1, 2, 3, 4, 7]))
    }
}

use std::usize;

use crate::board_cell::BoardCell;

pub struct Board {
    cells: Vec<Vec<BoardCell>>
}

impl Board {
    pub fn new(size: usize) -> Self {
        if size <= 0 {
            panic!("Uh oh! Please provide a board size greater than 0.");
        }

        let mut cells = Vec::new();

        (0..size).for_each(|i| {
            cells.push(Vec::new());
            (0..size).for_each(|_j| {
                cells[i].push(BoardCell::Unknown);
            });
        });

        Self {
            cells
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> BoardCell {
        self.cells[y][x]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, new_value: BoardCell) {
        self.cells[y][x] = new_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let board = Board::new(8);
        assert_eq!(board.cells.len(), 8);
        assert_eq!(board.cells[0][0], BoardCell::Unknown);
    }

    #[test]
    #[should_panic(expected = "Uh oh! Please provide a board size greater than 0.")]
    fn test_new_size_zero() {
        Board::new(0);
    }

    #[test]
    fn test_get_cells() {
        let board = Board::new(8);
        let cell = board.get_cell(0, 0);

        assert_eq!(cell, BoardCell::Unknown);
    }

    #[test]
    fn test_set_cell() {
        let mut board = Board::new(8);

        assert_eq!(board.cells[0][0], BoardCell::Unknown);
        board.set_cell(0, 0, BoardCell::Hit);
        assert_eq!(board.cells[0][0], BoardCell::Hit);
    }
}

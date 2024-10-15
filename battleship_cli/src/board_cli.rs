use std::fmt;
use battleship::board::Board;
use crate::cell_cli::CellCli;
use crate::shot_cli::ShotCli;

pub struct BoardCli {
    pub board: Board
}

impl BoardCli {
    pub fn new(board: Board) -> BoardCli {
        BoardCli {
            board
        }
    }

    pub fn shoot(&mut self, x: &str, y: &str) -> ShotCli {
        let x: u32 = x.parse().expect("An integer coordinate");
        let y: u32 = y.parse().expect("An integer coordinate");
        ShotCli::new(self.board.shoot(x, y))
    }
}

impl fmt::Display for BoardCli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = "   0 1 2 3 4 5 6 7 \n-------------------".to_owned();
        let mut count = 0;
        for row in &self.board.cells {
            board.push_str(&format!("\n{} |", count));
            for cell in row {
                board.push_str(&CellCli::new(*cell).to_string());
                board.push_str("|");
            }
            count += 1;
        }
        write!(f, "{board}")
    }
}
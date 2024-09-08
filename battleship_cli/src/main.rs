use std::fmt;
use battleship::{
    board::Board,
    cell::Cell
};

fn main() {
    let mut board_cli = BoardCli::new(Board::new(8));
    board_cli.board.start_game();

    println!("{}", board_cli);

    board_cli.board.shoot(0, 1);
    println!("{}", board_cli);

    board_cli.board.shoot(1, 1);
    println!("{}", board_cli);

    board_cli.board.shoot(1, 2);
    println!("{}", board_cli);

    board_cli.board.shoot(1, 3);
    println!("{}", board_cli);

    board_cli.board.shoot(1, 4);
    println!("{}", board_cli);
}

struct CellCli {
  cell: Cell
}

impl CellCli {
  pub fn new(cell: Cell) -> CellCli {
    CellCli {
      cell
    }
  }
}

impl fmt::Display for CellCli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char = match self.cell {
            Cell::Empty => " ",
            Cell::Hit => "X",
            Cell::Miss => "O"
        };
        write!(f, "{char}")
    }
}

struct BoardCli {
  board: Board
}

impl BoardCli {
  pub fn new(board: Board) -> BoardCli {
    BoardCli {
      board
    }
  }
}

impl fmt::Display for BoardCli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = "   0 1 2 3 4 5 6 7 \n-------------------".to_owned();
        let mut count = 0;
        for row in &self.board.hits_and_misses {
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

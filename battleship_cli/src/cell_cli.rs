use std::fmt;
use battleship::cell::Cell;

pub struct CellCli {
    pub cell: Cell
}

impl CellCli {
    pub fn new(cell: Cell) -> CellCli {
        CellCli {
            cell
        }
    }
}

// TODO: add env vars so that ship renders as space or S
impl fmt::Display for CellCli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char = match self.cell {
            Cell::Empty => " ",
            Cell::Hit => "X",
            Cell::Miss => "O",
            Cell::Ship => "S"
        };
        write!(f, "{char}")
    }
}
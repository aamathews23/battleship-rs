use std::{
    fmt,
    env
};
use battleship::cell::{
    Cell,
    CellType
};

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

impl fmt::Display for CellCli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cell_debug: bool = match env::var("CELL_DEBUG") {
            Ok(value) => if value == "1" { true } else { false },
            Err(_) => false
        };
        let char = match self.cell.cell_type {
            CellType::Empty => " ",
            CellType::Hit => "X",
            CellType::Miss => "O",
            CellType::Ship => if cell_debug { "S" } else { " " }
        };
        write!(f, "{char}")
    }
}
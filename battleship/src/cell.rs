
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Hit,
    Miss,
    Ship
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
    pub ship_idx: usize
}

impl Cell {
    pub fn new(cell_type: CellType, ship_idx: usize) -> Cell {
        Self {
            cell_type,
            ship_idx
        }
    }
}

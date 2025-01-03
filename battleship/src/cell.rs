
pub struct Cell {
    /// The type of a cell. Possible values: -1 = miss, 0 = empty, 1 = ship, 2 = hit
    pub cell_type: i32,
    /// The index of the ship in the game's ship list
    pub ship_idx: usize
}

impl Cell {
    pub fn new(cell_type: i32, ship_idx: usize) -> Cell {
        Self {
            cell_type,
            ship_idx
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cell = Cell::new(-1, 2);
        assert_eq!(cell.cell_type, -1);
        assert_eq!(cell.ship_idx, 2);
    }
}

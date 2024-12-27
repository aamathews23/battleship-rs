use crate::ship::Ship;

pub struct ShipYard {
    capacity: usize,
    ships: Vec<Ship>
}

/// A class for ship management in the battleship game.
impl ShipYard {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            ships: Vec::new()
        }
    }

    fn build_ship(&mut self, size: i32) -> bool {
        if !self.has_capacity() {
            return false;
        }

        let ship = Ship::new(size);
        self.ships.push(ship);

        true
    }

    pub fn build_destroyer(&mut self) -> bool {
        self.build_ship(2)
    }

    pub fn build_cruiser(&mut self) -> bool {
        self.build_ship(3)
    }

    pub fn build_battleship(&mut self) -> bool {
        self.build_ship(4)
    }

    pub fn get_ship(&mut self, idx: usize) -> &mut Ship {
        if self.get_yard_size() == 0 {
            panic!("Uh oh! There are no ships in the yard...");
        }

        if idx >= self.get_yard_size() {
            panic!("Uh oh! The ship you are requesting does not exist...")
        }

        &mut self.ships[idx]
    }

    pub fn get_yard_size(&self) -> usize {
        self.ships.len()
    }

    pub fn has_capacity(&self) -> bool {
        self.ships.len() < self.capacity
    }

    pub fn are_all_ships_sunk(&self) -> bool {
        if self.get_yard_size() == 0 {
            panic!("Uh oh! There are no ships in the yard...");
        }

        for ship in &self.ships {
            if ship.health > 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_ship_yard() -> ShipYard {
        ShipYard::new(3)
    }

    #[test]
    fn test_new() {
        let ship_yard = build_ship_yard();
        assert_eq!(ship_yard.ships.len(), 0);
    }

    #[test]
    fn test_build_ship_when_empty() {
        let mut ship_yard = build_ship_yard();
        let result = ship_yard.build_ship(1);

        assert!(result);
    }

    #[test]
    fn test_build_ship_when_occupied() {
        let mut ship_yard = build_ship_yard();
        let result = ship_yard.build_ship(1);

        assert!(result);
    }

    #[test]
    fn test_build_ship_when_full() {
        let mut ship_yard = build_ship_yard();
        let result1 = ship_yard.build_ship(1);
        let result2 = ship_yard.build_ship(1);
        let result3 = ship_yard.build_ship(1);
        let result4 = ship_yard.build_ship(1);

        assert!(result1);
        assert!(result2);
        assert!(result3);
        assert!(!result4);
    }

    #[test]
    fn test_build_destroyer() {
        let mut ship_yard = build_ship_yard();
        
        assert!(ship_yard.build_destroyer());
        assert_eq!(ship_yard.ships.len(), 1);
        assert_eq!(ship_yard.ships[0].size, 2);
    }

    #[test]
    fn test_build_cruiser() {
        let mut ship_yard = build_ship_yard();
        
        assert!(ship_yard.build_cruiser());
        assert_eq!(ship_yard.ships.len(), 1);
        assert_eq!(ship_yard.ships[0].size, 3);
    }

    #[test]
    fn test_build_battleship() {
        let mut ship_yard = build_ship_yard();
        
        assert!(ship_yard.build_battleship());
        assert_eq!(ship_yard.ships.len(), 1);
        assert_eq!(ship_yard.ships[0].size, 4);
    }

    #[test]
    fn test_get_ship() {
        let mut ship_yard = build_ship_yard();
        ship_yard.build_destroyer();
        assert_eq!(ship_yard.get_ship(0).size, 2);
    }

    #[test]
    #[should_panic = "Uh oh! There are no ships in the yard..."]
    fn test_get_ship_empty_yard() {
        let mut ship_yard = build_ship_yard();
        ship_yard.get_ship(0);
    }

    #[test]
    #[should_panic = "Uh oh! The ship you are requesting does not exist..."]
    fn test_get_ship_index_out_of_bounds_at_length() {
        let mut ship_yard = build_ship_yard();
        ship_yard.build_destroyer();
        ship_yard.get_ship(1);
    }

    #[test]
    #[should_panic = "Uh oh! The ship you are requesting does not exist..."]
    fn test_get_ship_index_out_of_bounds_greater_than_length() {
        let mut ship_yard = build_ship_yard();
        ship_yard.build_destroyer();
        ship_yard.get_ship(2);
    }

    #[test]
    fn test_get_yard_size() {
        let mut ship_yard = build_ship_yard();
        assert_eq!(ship_yard.get_yard_size(), 0);
        ship_yard.build_destroyer();
        assert_eq!(ship_yard.get_yard_size(), 1);
    }

    #[test]
    fn test_has_capacity() {
        let mut ship_yard = build_ship_yard();
        assert!(ship_yard.has_capacity());
        ship_yard.build_destroyer();
        ship_yard.build_cruiser();
        ship_yard.build_battleship();
        assert!(!ship_yard.has_capacity());
    }

    #[test]
    fn test_are_all_ships_sunk() {
        let mut ship_yard = build_ship_yard();
        ship_yard.build_destroyer();
        assert!(!ship_yard.are_all_ships_sunk());
        ship_yard.ships[0].hit();
        ship_yard.ships[0].hit();
        assert!(ship_yard.are_all_ships_sunk());
    }

    #[test]
    #[should_panic]
    fn test_are_all_ships_sunk_empty_yard() {
        let ship_yard = build_ship_yard();
        ship_yard.are_all_ships_sunk();
    }
}
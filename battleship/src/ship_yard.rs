use crate::ship::Ship;

pub struct ShipYard {
    ships: Vec<Ship>
}

impl ShipYard {
    pub fn new() -> Self {
        Self {
            ships: Vec::new()
        }
    }

    fn build_ship(&mut self, size: i32) {
        let ship = Ship::new(size);
        self.ships.push(ship);
    }

    pub fn build_destroyer(&mut self) {
        self.build_ship(2);
    }

    pub fn build_cruiser(&mut self) {
        self.build_ship(3);
    }

    pub fn build_battleship(&mut self) {
        self.build_ship(4);
    }

    pub fn get_ship(&mut self, idx: usize) -> &mut Ship {
        &mut self.ships[idx]
    }

    pub fn get_yard_size(&self) -> usize {
        self.ships.len()
    }

    pub fn has_ships(&self) -> bool {
        self.ships.len() > 0
    }

    pub fn has_capacity(&self) -> bool {
        self.ships.len() < 3
    }

    pub fn are_all_ships_sunk(&self) -> bool {
        for ship in &self.ships {
            if ship.health > 0 {
                return false;
            }
        }

        true
    }
}
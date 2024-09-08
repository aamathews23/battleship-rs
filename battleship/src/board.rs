use crate::{
    cell::Cell,
    ship::{
        Ship,
        ShipSize
    },
    shot
};

#[derive(Debug)]
pub struct Board {
    pub size: u32,
    pub hits_and_misses: Vec<Vec<Cell>>,
    pub ships: Vec<Ship>
}

impl Board {
    pub fn new(size: u32) -> Board {
        let hits_and_misses: Vec<Vec<Cell>> = Vec::new();
        let ships: Vec<Ship> = Vec::new();
        Board {
            size,
            hits_and_misses,
            ships
        }
    }

    pub fn start_game(&mut self) {
        (0..self.size as usize).for_each(|i| {
            self.hits_and_misses.push(Vec::new());
            (0..self.size as usize).for_each(|_j| {
                self.hits_and_misses[i].push(Cell::Empty);
            });
        });
        self.ships.push(self.place_ship(ShipSize::Destroyer));
        self.ships.push(self.place_ship(ShipSize::Cruiser));
        self.ships.push(self.place_ship(ShipSize::Battleship));
    }

    pub fn shoot(&mut self, x: u32, y: u32) -> shot::Result {
        if x > self.size - 1 || y > self.size - 1 {
            return shot::Result::new(shot::Outcome::TryAgain, self.hits_and_misses.clone(), self.ships.clone());
        }

        for ship in &mut self.ships {
            if ship.hit(x, y) {
                self.hits_and_misses[y as usize][x as usize] = Cell::Hit;
                return shot::Result::new(shot::Outcome::Hit, self.hits_and_misses.clone(), self.ships.clone());
            }
        }

        self.hits_and_misses[y as usize][x as usize] = Cell::Miss;
        shot::Result::new(shot::Outcome::Miss, self.hits_and_misses.clone(), self.ships.clone())
    }

    fn valid_ship(&self, ship: Ship) -> bool {
        for s in &self.ships {
            if ship.start.x <= s.start.x && ship.end.x >= s.start.x && ship.start.y >= s.start.y && ship.start.y <= s.end.y {
                return false;
            }
            if ship.start.y <= s.start.y && ship.end.y >= s.start.y && ship.start.x >= s.start.x && ship.start.x <= s.end.x {
                return false;
            }
        }

        true
    }

    fn place_ship(&self, ship_size: ShipSize) -> Ship {
        let mut ship = Ship::new(0, self.size - 1, ship_size);

        while !self.valid_ship(ship) {
            ship.place(0, self.size - 1);
        }

        ship
    }
}
use crate::{
    cell::Cell, direction::Direction, ship::{
        Ship,
        ShipSize
    },
    shot::Shot
};

#[derive(Debug)]
pub struct Board {
    pub size: u32,
    pub cells: Vec<Vec<Cell>>,
    pub ships: Vec<Ship>
}

impl Board {
    pub fn new(size: u32) -> Board {
        let cells: Vec<Vec<Cell>> = Vec::new();
        let ships: Vec<Ship> = Vec::new();
        Board {
            size,
            cells,
            ships
        }
    }

    pub fn start_game(&mut self) {
        (0..self.size as usize).for_each(|i| {
            self.cells.push(Vec::new());
            (0..self.size as usize).for_each(|_j| {
                self.cells[i].push(Cell::Empty);
            });
        });

        let destoryer = self.place_ship(ShipSize::Destroyer);
        let cruiser = self.place_ship(ShipSize::Cruiser);
        let battleship = self.place_ship(ShipSize::Battleship);

        self.ships.push(destoryer);
        self.ships.push(cruiser);
        self.ships.push(battleship);
    }

    pub fn shoot(&mut self, x: u32, y: u32) -> Shot {
        if x > self.size - 1 || y > self.size - 1 {
            return Shot::TryAgain;
        }

        if self.cells[y as usize][x as usize] == Cell::Ship {
            self.cells[y as usize][x as usize] = Cell::Hit;
            return Shot::Hit;
        }

        self.cells[y as usize][x as usize] = Cell::Miss;
        Shot::Miss
    }

    fn valid_ship(&self, ship: Ship) -> bool {
        let range;
        let static_axis;

        match ship.direction {
            Direction::Horizontal => {
                range = ship.start.x..ship.end.x;
                static_axis = ship.start.y as usize;
            },
            Direction::Vertical => {
                range = ship.start.y..ship.end.y;
                static_axis = ship.start.x as usize;
            },
        };

        match ship.direction {
            Direction::Horizontal => {
                for i in range {
                    println!("({static_axis}, {i})");
                    if self.cells[static_axis][i as usize] == Cell::Ship {
                        return false;
                    }
                }
            },
            Direction::Vertical => {
                for i in range {
                    println!("({i}, {static_axis})");
                    if self.cells[i as usize][static_axis] == Cell::Ship {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn place_ship(&mut self, ship_size: ShipSize) -> Ship {
        let mut ship = Ship::new(0, self.size - 1, ship_size);

        while !self.valid_ship(ship) {
            ship.place(0, self.size - 1);
        }

        match ship.direction {
            Direction::Horizontal => {
                (ship.start.x..ship.end.x).for_each(|x| {
                    self.cells[ship.start.y as usize][x as usize] = Cell::Ship;
                });
            },
            Direction::Vertical => {
                (ship.start.y..ship.end.y).for_each(|y| {
                    self.cells[y as usize][ship.start.x as usize] = Cell::Ship;
                });
            }
        }

        ship
    }
}
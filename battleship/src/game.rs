use crate::{
    board::Board,
    ship::{
        Ship,
        ShipSize
    },
    shot::Shot
};

pub struct Game {
    pub amt_of_turns: u32,
    pub amt_of_hits: u32,
    pub amt_of_misses: u32,
    pub ships_sunk: u32,
    pub board: Board,
    pub ships: Vec<Ship>
}

impl Game {
    pub fn new() -> Self {
        Self {
            amt_of_turns: 0,
            amt_of_hits: 0,
            amt_of_misses: 0,
            ships_sunk: 0,
            board: Board::new(),
            ships: Vec::new()
        }
    }

    pub fn start_game(&mut self, size: u32) {
        let destoryer = Ship::new(ShipSize::Destroyer);
        let cruiser = Ship::new(ShipSize::Cruiser);
        let battleship = Ship::new(ShipSize::Battleship);
        self.ships.push(destoryer);
        self.ships.push(cruiser);
        self.ships.push(battleship);

        self.board.init_board(size, &self.ships);
    }

    pub fn shoot(&mut self, x: u32, y: u32) -> Shot {
        self.amt_of_turns += 1;

        let shot_res = self.board.shoot(x, y);

        if shot_res == -2 {
            self.amt_of_misses += 1;
            return Shot::Miss;
        }

        self.amt_of_hits += 1;

        if shot_res == -1 {
            return Shot::Hit;
        }

        let ship = &mut self.ships[shot_res as usize];
        ship.hit();

        if ship.health == 0 {
            self.ships_sunk += 1;

            return Shot::Sunk;
        }

        return Shot::Hit;
    }

    pub fn is_end(&self) -> bool {
        for ship in &self.ships {
            if ship.health > 0 {
                return false;
            }
        }

        true
    }
}

use std::usize;

use rand::{
    seq::IteratorRandom,
    thread_rng
};

use crate::{
    cell::{
        Cell,
        CellType
    },
    direction::Direction,
    ship::Ship
};

#[derive(Debug)]
pub struct Board {
    pub size: u32,
    pub cells: Vec<Vec<Cell>>
}

impl Board {
    pub fn new() -> Board {
        Board {
            size: 0,
            cells: Vec::new()
        }
    }

    pub fn init_board(&mut self, size: u32, ships: &Vec<Ship>) {
        self.size = size;
        self.init_empty_board();
        self.init_ships_on_board(ships);
    }

    fn init_empty_board(&mut self) {
        (0..self.size as usize).for_each(|i| {
            self.cells.push(Vec::new());
            (0..self.size as usize).for_each(|_j| {
                let cell = Cell::new(CellType::Empty, usize::MAX);
                self.cells[i].push(cell);
            });
        });
    }

    fn init_ships_on_board(&mut self, ships: &Vec<Ship>) {
        for i in 0..ships.len() {
            let ship = ships[i];
            let direction = ship.direction;
            let ship_size: u32 = ship.size.into();
            let mut s = self.get_random_index();
            let mut e = s + ship_size;
            let mut static_idx = self.get_random_index();

            while !self.is_ship_valid(direction, s, e, static_idx) {
                s = self.get_random_index();
                e = s + ship_size;
                static_idx = self.get_random_index();
            }

            match direction {
                Direction::Horizontal => {
                    for x in s..e {
                        let cell = &mut self.cells[static_idx as usize][x as usize];
                        cell.cell_type = CellType::Ship;
                        cell.ship_idx = i;
                    }
                },
                Direction::Vertical => {
                    for y in s..e {
                        let cell = &mut self.cells[y as usize][static_idx as usize];
                        cell.cell_type = CellType::Ship;
                        cell.ship_idx = i;
                    }
                }
            }
        }
    }

    fn get_random_index(&self) -> u32 {
        let choices = 0..self.size;
        let mut rng = thread_rng();
        let idx = match choices.choose(&mut rng) {
            Some(num) => num,
            _ => 0
        };
        idx
    }

    fn is_ship_valid(&self, direction: Direction, s: u32, e: u32, static_idx: u32) -> bool {
        if e > self.size - 1 {
            return false;
        }

        match direction {
            Direction::Horizontal => {
                for i in s..e {
                    if self.cells[static_idx as usize][i as usize].cell_type == CellType::Ship {
                        return false;
                    }
                }
            },
            Direction::Vertical => {
                for i in s..e {
                    if self.cells[i as usize][static_idx as usize].cell_type == CellType::Ship {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn shoot(&mut self, x: u32, y: u32) -> i32 {
        let cell = &mut self.cells[y as usize][x as usize];

        if cell.cell_type == CellType::Ship {
            cell.cell_type = CellType::Hit;
            return cell.ship_idx as i32;
        }

        if cell.cell_type == CellType::Hit {
            return -1;
        }

        cell.cell_type = CellType::Miss;
        -2
    }
}
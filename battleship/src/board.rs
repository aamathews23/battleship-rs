use std::usize;

use crate::{
    cell::Cell,
    direction::Direction,
    random_generator::RandomGenerator,
    ship_yard::ShipYard,
    shoot_trait::ShootTrait
};

pub struct Board {
    pub size: i32,
    pub cells: Vec<Vec<Cell>>
}

impl Board {
    pub fn new(size: i32) -> Self {
        if size <= 0 {
            panic!("Uh oh! Please provide a board size greater than 0.");
        }

        Self {
            size,
            cells: Vec::new()
        }
    }

    pub fn init_board(&mut self, ship_yard: &mut ShipYard, generator: &mut dyn RandomGenerator) {
        (0..self.size as usize).for_each(|i| {
            self.cells.push(Vec::new());
            (0..self.size as usize).for_each(|_j| {
                let cell = Cell::new(0, usize::MAX);
                self.cells[i].push(cell);
            });
        });

        for i in 0..ship_yard.get_yard_size() {
            let ship = ship_yard.get_ship(i).expect("a ship to be present in the ship yard");
            let direction = &ship.direction;
            let ship_size: i32 = ship.size.into();
            let mut s = generator.generate(0, self.size);
            let mut e = s + ship_size;
            let mut static_idx = generator.generate(0, self.size);

            while !Board::is_ship_valid(&self, direction, s, e, static_idx) {
                s = generator.generate(0, self.size);
                e = s + ship_size;
                static_idx = generator.generate(0, self.size);
            }

            match direction {
                Direction::Horizontal => {
                    for x in s..e {
                        let cell = &mut self.cells[static_idx as usize][x as usize];
                        cell.cell_type = 1;
                        cell.ship_idx = i;
                    }
                },
                Direction::Vertical => {
                    for y in s..e {
                        let cell = &mut self.cells[y as usize][static_idx as usize];
                        cell.cell_type = 1;
                        cell.ship_idx = i;
                    }
                }
            }
        }
    }

    fn is_ship_valid(board: &Self, direction: &Direction, s: i32, e: i32, static_idx: i32) -> bool {
        if e > board.size - 1 {
            return false;
        }

        match direction {
            Direction::Horizontal => {
                for i in s..e {
                    if board.cells[static_idx as usize][i as usize].cell_type == 1 {
                        return false;
                    }
                }
            },
            Direction::Vertical => {
                for i in s..e {
                    if board.cells[i as usize][static_idx as usize].cell_type == 1 {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl ShootTrait for Board {
    /// Determines the result of a shot from the user. Possible values: -2 = Miss, -1 = Repeat hit, > -1 = ship index
    fn shoot(&mut self, x: i32, y: i32) -> i32 {
        let cell = &mut self.cells[y as usize][x as usize];

        if cell.cell_type == 1 {
            cell.cell_type = 2;
            return cell.ship_idx as i32;
        }

        if cell.cell_type == 2 {
            return -1;
        }

        cell.cell_type = -1;
        -2
    }
}

#[cfg(test)]
mod tests {
    use crate::random_generator::MockRandomGenerator;

    use super::*;

    fn init_board_helper(size: i32, has_ship: bool) -> Board {
        let mut ship_yard = ShipYard::new(3);
        
        let mut mock = MockRandomGenerator::new();
        mock.expect_generate()
            .returning(|_s, _e| 0);

        if has_ship {
            ship_yard.build_destroyer();
        }

        let mut board = Board::new(size);
        board.init_board(&mut ship_yard, &mut mock);

        board
    }

    #[test]
    fn test_new() {
        let board = Board::new(8);
        assert_eq!(board.size, 8);
        assert_eq!(board.cells.len(), 0);
    }

    #[test]
    #[should_panic(expected = "Uh oh! Please provide a board size greater than 0.")]
    fn test_new_size_zero() {
        Board::new(0);
    }

    #[test]
    fn test_init_board() {
        let board = init_board_helper(8, true);

        assert_eq!(board.cells[0][0].cell_type, 1);
    }

    #[test]
    fn test_is_ship_valid_success() {
        let board = init_board_helper(8, true);
        let is_valid = Board::is_ship_valid(&board, &Direction::Horizontal, 1, 1, 0);
        assert!(is_valid);
    }

    #[test]
    fn test_is_ship_valid_failure() {
        let board = init_board_helper(8, true);
        let is_valid = Board::is_ship_valid(&board,&Direction::Horizontal, 0, 1, 0);
        assert!(!is_valid);
    }

    #[test]
    fn test_shoot_hit() {
        let mut board = init_board_helper(8, true);
        let hit_result = board.shoot(0, 0);
        assert_eq!(hit_result, 0);
    }

    #[test]
    fn test_shoot_repeat_hit() {
        let mut board = init_board_helper(8, true);
        board.shoot(0, 0);
        let hit_result = board.shoot(0, 0);
        assert_eq!(hit_result, -1);
    }

    #[test]
    fn test_shoot_repeat_miss() {
        let mut board = init_board_helper(8, true);
        let hit_result = board.shoot(0, 1);
        assert_eq!(hit_result, -2);
    }
}

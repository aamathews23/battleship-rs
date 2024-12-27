use crate::{
    board::Board,
    game_trait::GameTrait,
    random_generator::RandomGeneratorImpl,
    random_generator_trait::RandomGeneratorTrait,
    ship_yard::ShipYard,
    shoot_trait::ShootTrait
};

pub struct Game {
    pub amt_of_turns: i32,
    pub amt_of_hits: i32,
    pub amt_of_misses: i32,
    pub ships_sunk: i32,
    pub board: Board,
    ship_yard: ShipYard
}

impl Game {
    pub fn new(size: i32) -> Self {
        Self {
            amt_of_turns: 0,
            amt_of_hits: 0,
            amt_of_misses: 0,
            ships_sunk: 0,
            board: Board::new(size),
            ship_yard: ShipYard::new(3)
        }
    }

    pub fn add_destroyer(&mut self) {
        self.ship_yard.build_destroyer();
    }

    pub fn add_cruiser(&mut self) {
        self.ship_yard.build_cruiser();
    }

    pub fn add_battleship(&mut self) {
        self.ship_yard.build_battleship();
    }

    pub fn is_end(&self) -> bool {
        self.ship_yard.are_all_ships_sunk()
    }

    fn sg(&mut self, generator: &mut dyn RandomGeneratorTrait) {
        self.board.init_board(&mut self.ship_yard, generator);
    }
}

impl ShootTrait for Game {
    /// Determines if a ship was hit. Possible values: -1 = Miss, 0 = Hit, 1 = Ship sunk
    fn shoot(&mut self, x: i32, y: i32) -> i32 {
        self.amt_of_turns += 1;

        let shot_res = self.board.shoot(x, y);

        if shot_res == -2 {
            self.amt_of_misses += 1;
            return -1;
        }

        if shot_res == -1 {
            return 0;
        }

        self.amt_of_hits += 1;

        let ship = self.ship_yard.get_ship(shot_res as usize);
        ship.hit();

        if ship.health == 0 {
            self.ships_sunk += 1;

            return 1;
        }

        return 0;
    }
}

impl GameTrait for Game {
    fn start_game(&mut self) {
        let mut generator = RandomGeneratorImpl::new();
        self.sg(&mut generator);
    }
}

#[cfg(test)]
mod tests {
    use crate::random_generator_trait::MockRandomGeneratorTrait;

    use super::*;

    fn init_game_helper() -> Game {
        let mut game = Game::new(8);
        game.add_destroyer();
        game
    }

    #[test]
    fn test_new() {
        let game = init_game_helper();
        assert_eq!(game.amt_of_hits, 0);
        assert_eq!(game.amt_of_misses, 0);
        assert_eq!(game.amt_of_turns, 0);
        assert_eq!(game.ships_sunk, 0);
        assert_eq!(game.board.size, 8);
        assert_eq!(game.board.cells.len(), 0);
        assert_eq!(game.is_end(), false);
    }

    #[test]
    fn test_start_game() {
        let mut mock = MockRandomGeneratorTrait::new();
        mock.expect_generate()
            .returning(|_s, _e| 0);

        let mut game = init_game_helper();

        game.sg(&mut mock);

        assert_eq!(game.amt_of_hits, 0);
        assert_eq!(game.amt_of_misses, 0);
        assert_eq!(game.amt_of_turns, 0);
        assert_eq!(game.ships_sunk, 0);
        assert_eq!(game.board.cells.len(), 8);
        assert_eq!(game.board.cells[0][0].cell_type, 1);
    }

    #[test]
    fn test_shoot_hit() {
        let mut mock = MockRandomGeneratorTrait::new();
        mock.expect_generate()
            .returning(|_s, _e| 0);

        let mut game = init_game_helper();

        game.sg(&mut mock);

        game.shoot(0, 0);

        assert_eq!(game.amt_of_hits, 1);
        assert_eq!(game.amt_of_misses, 0);
        assert_eq!(game.amt_of_turns, 1);
        assert_eq!(game.ships_sunk, 0);
        assert_eq!(game.board.cells[0][0].cell_type, 2);
    }

    #[test]
    fn test_shoot_repeat_hit() {
        let mut mock = MockRandomGeneratorTrait::new();
        mock.expect_generate()
            .returning(|_s, _e| 0);

        let mut game = init_game_helper();

        game.sg(&mut mock);

        game.shoot(0, 0);
        game.shoot(0, 0);

        assert_eq!(game.amt_of_hits, 1);
        assert_eq!(game.amt_of_misses, 0);
        assert_eq!(game.amt_of_turns, 2);
        assert_eq!(game.ships_sunk, 0);
        assert_eq!(game.board.cells[0][0].cell_type, 2);
    }

    #[test]
    fn test_shoot_sink() {
        let mut mock = MockRandomGeneratorTrait::new();
        mock.expect_generate()
            .returning(|_s, _e| 0);

        let mut game = init_game_helper();

        game.sg(&mut mock);

        game.shoot(0, 0);
        game.shoot(1, 0);

        assert_eq!(game.amt_of_hits, 2);
        assert_eq!(game.amt_of_misses, 0);
        assert_eq!(game.amt_of_turns, 2);
        assert_eq!(game.ships_sunk, 1);
        assert_eq!(game.board.cells[0][0].cell_type, 2);
        assert_eq!(game.board.cells[0][1].cell_type, 2);
    }

    #[test]
    fn test_shoot_miss() {
        let mut mock = MockRandomGeneratorTrait::new();
        mock.expect_generate()
            .returning(|_s, _e| 0);

        let mut game = init_game_helper();

        game.sg(&mut mock);

        game.shoot(0, 1);

        assert_eq!(game.amt_of_hits, 0);
        assert_eq!(game.amt_of_misses, 1);
        assert_eq!(game.amt_of_turns, 1);
        assert_eq!(game.ships_sunk, 0);
        assert_eq!(game.board.cells[0][0].cell_type, 1);
        assert_eq!(game.board.cells[1][0].cell_type, -1);
    }

    #[test]
    fn test_is_end_after_init() {
        let game = init_game_helper();

        assert_eq!(game.is_end(), false);
    }

    #[test]
    fn test_is_end_after_start_game() {
        let mut mock = MockRandomGeneratorTrait::new();
        mock.expect_generate()
            .returning(|_s, _e| 0);

        let mut game = init_game_helper();
        game.sg(&mut mock);


        assert_eq!(game.is_end(), false);
    }

    #[test]
    fn test_is_end_after_one_hit() {
        let mut mock = MockRandomGeneratorTrait::new();
        mock.expect_generate()
            .returning(|_s, _e| 0);

        let mut game = init_game_helper();
        game.sg(&mut mock);

        game.shoot(0, 0);

        assert_eq!(game.is_end(), false);
    }

    #[test]
    fn test_is_end_after_sink() {
        let mut mock = MockRandomGeneratorTrait::new();
        mock.expect_generate()
            .returning(|_s, _e| 0);

        let mut game = init_game_helper();
        game.sg(&mut mock);

        game.shoot(0, 0);
        game.shoot(1, 0);

        assert_eq!(game.is_end(), true);
    }
}

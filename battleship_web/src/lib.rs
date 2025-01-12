use wasm_bindgen::prelude::*;

use battleship::{
    board_cell::BoardCell,
    game::Game,
    game_trait::GameTrait, shoot_trait::{ShootTrait, ShootTraitResult}
};

#[wasm_bindgen]
pub struct BattleshipWeb {
    game: Game,
    board: Vec<BoardCell>
}

#[wasm_bindgen]
impl BattleshipWeb {
    pub fn new() -> Self {
        let mut game = Game::new(8);
        game.start_game();

        let mut cells = Self::flatten_board();

        Self {
            game,
            board: cells
        }
    }

    pub fn amt_of_turns(&self) -> u32 {
        self.game.amt_of_turns
    }

    pub fn amt_of_hits(&self) -> u32 {
        self.game.amt_of_hits
    }

    pub fn amt_of_misses(&self) -> u32 {
        self.game.amt_of_misses
    }

    pub fn ships_sunk(&self) -> u32 {
        self.game.ships_sunk
    }

    pub fn is_end(&self) -> bool {
        self.game.is_end()
    }

    pub fn board(&self) -> *const BoardCell {
        self.board.as_ptr()
    }

    pub fn shoot(&mut self, x: u32, y: u32) -> u32 {
        let shot_result = self.game.shoot(x, y);

        return match shot_result {
            ShootTraitResult::Miss => 0,
            ShootTraitResult::Hit => 1,
            ShootTraitResult::Sunk => 2,
            ShootTraitResult::Repeat => 3
        };
    }
}

impl BattleshipWeb {
    fn flatten_board(&self) -> Vec<BoardCell> {
        let mut cells: Vec<BoardCell> = Vec::new();

        for y in 0..8 {
            for x in 0..8 {
                cells.push(self.game.board.get_cell(x, y));
            }
        }

        cells
    }
}

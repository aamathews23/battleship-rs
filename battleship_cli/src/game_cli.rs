use battleship::{
    board_cell::BoardCell, game::Game, game_trait::GameTrait, shoot_trait::{ShootTrait, ShootTraitResult}
};
use regex::Regex;
use std::{
    env,
    io::{stdin, stdout, Write},
};

#[derive(Debug, PartialEq)]
pub enum GameInputValidationType {
    Coords(Vec<u32>),
    Stats,
    Quit,
    InvalidInput,
}

pub struct GameCli {
    game: Game,
}

impl GameCli {
    pub fn new() -> Self {
        let mut game = Game::new(8);
        game.start_game();
        Self { game }
    }

    fn get_user_input(s: &str) -> GameInputValidationType {
        let re = Regex::new(r"^([A-H]|[a-h])([1-8])$").unwrap();

        if s == "stats" {
            return GameInputValidationType::Stats;
        }

        if s == "quit" {
            return GameInputValidationType::Quit;
        }

        if re.is_match(s) {
            let coords: Vec<&str> = s.split("").collect();
            let x = coords[1]
                .to_lowercase()
                .chars()
                .nth(0)
                .expect("a character") as u32
                - 97;
            let y = coords[2]
                .chars()
                .nth(0)
                .expect("a digit")
                .to_digit(10)
                .expect("a valid digit") as u32
                - 1;
            let mut res = Vec::new();
            res.push(x);
            res.push(y);
            return GameInputValidationType::Coords(res);
        }

        GameInputValidationType::InvalidInput
    }

    fn parse_user_input() -> GameInputValidationType {
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        Self::get_user_input(&s)
    }

    fn print_board(&self) {
        let mut board = "   A B C D E F G H \n-------------------".to_owned();
        let cell_debug: bool = match env::var("CELL_DEBUG") {
            Ok(value) => {
                if value == "1" {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        };
        for y in 0..8 {
            board.push_str(&format!("\n{} |", y + 1));
            for x in 0..8 {
                let cell = self.game.board.get_cell(x, y);
                let cell_string: &str = match cell {
                    BoardCell::Unknown => " ",
                    BoardCell::Ship => {
                        if cell_debug {
                            "S"
                        } else {
                            " "
                        }
                    },
                    BoardCell::Hit => "X",
                    BoardCell::Miss => "O"
                };
                board.push_str(cell_string);
                board.push_str("|");
            }
        }
        println!("{}", board);
    }

    fn print_shot(&mut self, x: u32, y: u32) {
        let shot = match self.game.shoot(x, y) {
            ShootTraitResult::Hit => "Hit.",
            ShootTraitResult::Miss => "Miss...",
            ShootTraitResult::Sunk => "Sunk!",
            ShootTraitResult::Repeat => "Try again."
        };
        println!("{}", shot);
    }

    fn print_stats(&self) {
        println!("# of turns: {}", self.game.amt_of_turns);
        println!("# of hits: {}", self.game.amt_of_hits);
        println!("# of misses: {}", self.game.amt_of_misses);
        println!("# of ships sunk: {}", self.game.ships_sunk);
    }
}

impl GameTrait for GameCli {
    fn start_game(&mut self) {
        loop {
            self.print_board();
            print!("\nWhere do you want to shoot? ");
            let input = Self::parse_user_input();

            match input {
                GameInputValidationType::Stats => {
                    println!();
                    println!("Here is your current game stats:");
                    self.print_stats();
                    println!();
                }
                GameInputValidationType::Quit => {
                    break;
                }
                GameInputValidationType::Coords(coords) => {
                    println!();
                    self.print_shot(coords[0], coords[1]);
                    println!();
                }
                GameInputValidationType::InvalidInput => {
                    println!("\nInvalid input, try again.\n")
                }
            }

            if self.game.is_end() {
                println!("Congrats! You've sunk all the ships.\n\nHere are your game stats:");
                self.print_stats();
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let game_cli = GameCli::new();
        assert_eq!(game_cli.game.amt_of_hits, 0);
        assert_eq!(game_cli.game.amt_of_misses, 0);
        assert_eq!(game_cli.game.amt_of_turns, 0);
        assert_eq!(game_cli.game.ships_sunk, 0);
        assert_eq!(game_cli.game.is_end(), false);
    }

    #[test]
    fn test_get_user_input_quit() {
        assert_eq!(GameCli::get_user_input("quit"), GameInputValidationType::Quit);
    }

    #[test]
    fn test_get_user_input_stats() {
        assert_eq!(GameCli::get_user_input("stats"), GameInputValidationType::Stats);
    }

    #[test]
    fn test_get_user_input_invalid() {
        assert_eq!(GameCli::get_user_input("invalid"), GameInputValidationType::InvalidInput);
    }

    #[test]
    fn test_get_user_input_coords_success() {
        assert_eq!(GameCli::get_user_input("A1"), GameInputValidationType::Coords([0, 0].to_vec()));
    }

    #[test]
    fn test_get_user_input_coords_invalid_x_coord() {
        assert_eq!(GameCli::get_user_input("Z1"), GameInputValidationType::InvalidInput);
    }


    #[test]
    fn test_get_user_input_coords_invalid_y_coord() {
        assert_eq!(GameCli::get_user_input("A9"), GameInputValidationType::InvalidInput);
    }
}
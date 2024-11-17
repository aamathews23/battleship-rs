use battleship::{
    game::Game,
    ship::{
        Ship,
        ShipSize
    },
    random_generator::RandomGenerator,
    game_trait::GameTrait,
    shoot_trait::ShootTrait
};
use regex::Regex;
use std::{env, io::{
    stdin,
    stdout,
    Write
}};

pub enum GameInputValidationType {
    Coords(Vec<i32>),
    Stats,
    Quit,
    InvalidInput
}

pub struct GameCli {
    pub game: Game
}

impl GameCli {
    pub fn new() -> Self {
        let mut ships = Vec::new();
        let destoryer = Ship::new(ShipSize::Destroyer);
        let cruiser = Ship::new(ShipSize::Cruiser);
        let battleship = Ship::new(ShipSize::Battleship);
        ships.push(destoryer);
        ships.push(cruiser);
        ships.push(battleship);
        Self {
            game: Game::new(8, ships)
        }
    }

    fn get_user_input() -> GameInputValidationType {
        let mut s = String::new();
        let re = Regex::new(r"^([A-H]|[a-h])([1-8])$").unwrap();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        if s == "stats" {
            return GameInputValidationType::Stats;
        }

        if s == "quit" {
            return GameInputValidationType::Quit;
        }
        
        if re.is_match(&s) {
            let coords: Vec<&str> = s.split("").collect();
            let x = coords[1]
                .to_lowercase().chars().nth(0).expect("a character") as i32
                - 97;
            let y = coords[2]
                .chars().nth(0).expect("a digit")
                .to_digit(10).expect("a valid digit") as i32
                - 1;
            let mut res = Vec::new();
            res.push(x);
            res.push(y);
            return GameInputValidationType::Coords(res);
        }

        GameInputValidationType::InvalidInput
    }

    fn print_board(&self) {
        let mut board = "   A B C D E F G H \n-------------------".to_owned();
        let mut count = 1;
        for row in &self.game.board.cells {
            board.push_str(&format!("\n{} |", count));
            for cell in row {
                let cell_debug: bool = match env::var("CELL_DEBUG") {
                    Ok(value) => if value == "1" { true } else { false },
                    Err(_) => false
                };
                let cell_string = match cell.cell_type {
                    -1 => "O",
                    1 => if cell_debug { "S" } else { " " },
                    2 => "X",
                    _ => " "
                };
                board.push_str(cell_string);
                board.push_str("|");
            }
            count += 1;
        }
        println!("{}", board);
    }

    fn print_shot(&mut self, x: i32, y: i32) {
        let shot = match self.game.shoot(x, y) {
            0 => "Hit.",
            1 => "Ship sunk!",
            _ => "Miss..."
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
    fn start_game(&mut self, generator: &mut dyn RandomGenerator) {
        self.game.start_game(generator);
        loop {
            self.print_board();
            print!("\nWhere do you want to shoot? ");
            let input = Self::get_user_input();

            match input {
                GameInputValidationType::Stats => {
                    println!();
                    println!("Here is your current game stats:");
                    self.print_stats();
                    println!();
                },
                GameInputValidationType::Quit => {
                    break;
                }
                GameInputValidationType::Coords(coords) => {
                    println!();
                    self.print_shot(coords[0], coords[1]);
                    println!();
                },
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

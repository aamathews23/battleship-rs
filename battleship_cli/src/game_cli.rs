use battleship::game::Game;
use regex::Regex;
use std::io::{
    stdin,
    stdout,
    Write
};
use crate::{
    cell_cli::CellCli,
    shot_cli::ShotCli
};

pub enum GameInputValidationType {
    Coords(Vec<u32>),
    Stats,
    Quit,
    InvalidInput
}

pub struct GameCli {
    pub game: Game
}

impl GameCli {
    pub fn new() -> Self {
        Self {
            game: Game::new()
        }
    }

    pub fn start_game(&mut self, size: u32) {
        self.game.start_game(size);
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

    fn get_user_input() -> GameInputValidationType {
        let mut s = String::new();
        let re = Regex::new(r"([A-H]|[a-h]) ([1-8])").unwrap();
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
            let coords: Vec<&str> = s.split(" ").collect();
            let x = coords[0]
                .to_lowercase().chars().nth(0).expect("a character") as u32
                - 97;
            let y = coords[1]
                .chars().nth(0).expect("a digit")
                .to_digit(10).expect("a valid digit")
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
                board.push_str(&CellCli::new(*cell).to_string());
                board.push_str("|");
            }
            count += 1;
        }
        println!("{}", board);
    }

    fn print_shot(&mut self, x: u32, y: u32) {
        let shot = ShotCli::new(self.game.shoot(x, y));
        println!("{}", shot);
    }

    fn print_stats(&self) {
        println!("# of turns: {}", self.game.amt_of_turns);
        println!("# of hits: {}", self.game.amt_of_hits);
        println!("# of misses: {}", self.game.amt_of_misses);
        println!("# of ships sunk: {}", self.game.ships_sunk);
    }
}

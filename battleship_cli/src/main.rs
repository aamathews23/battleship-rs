extern crate dotenv;

use battleship::game_trait::GameTrait;
use dotenv::dotenv;
use game_cli::GameCli;

mod game_cli;

fn main() {
    // TODO: Add pipeline build in Github
    dotenv().expect("dotenv loaded environment vars from .env");
    let mut game_cli = GameCli::new();

    game_cli.start_game();
}

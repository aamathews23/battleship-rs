extern crate dotenv;

use dotenv::dotenv;
use game_cli::GameCli;

mod cell_cli;
mod shot_cli;
mod game_cli;

fn main() {
    dotenv().expect("dotenv loaded environment vars from .env");
    let mut game_cli = GameCli::new();

    game_cli.start_game(8);
}

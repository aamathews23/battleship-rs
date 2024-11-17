extern crate dotenv;

use dotenv::dotenv;
use game_cli::GameCli;
use battleship::{
    game_trait::GameTrait,
    random_generator::RandomGeneratorImpl
};

mod game_cli;

fn main() {
    // TODO: Abstract RandomGenerator as a trait and RandomGeneratorImpl
    // TODO: Add integration tests for battleship pkg
    // TODO: Add unit tests for CLI funcs
    // TODO: Properly scope board in game
    // TODO: Properly scope game params
    // TODO: Add pipeline build in Github
    dotenv().expect("dotenv loaded environment vars from .env");
    let mut game_cli = GameCli::new();
    let mut generator = RandomGeneratorImpl::new();

    game_cli.start_game(&mut generator);
}

extern crate dotenv;

use dotenv::dotenv;
use game_cli::GameCli;
use battleship::{
    game_trait::GameTrait,
    random_generator::RandomGeneratorImpl
};

mod game_cli;

fn main() {
    
    // TODO: Add unit tests, add integration tests, add regression tests
    // TODO: Add pipeline build in Github
    dotenv().expect("dotenv loaded environment vars from .env");
    let mut game_cli = GameCli::new();
    let mut generator = RandomGeneratorImpl::new();

    game_cli.start_game(&mut generator);
}

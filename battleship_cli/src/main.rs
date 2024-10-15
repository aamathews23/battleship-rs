mod board_cli;
mod cell_cli;
mod shot_cli;

use std::io::{stdin,stdout,Write};
use battleship::board::Board;
use board_cli::BoardCli;

fn main() {
    let mut board_cli = BoardCli::new(Board::new(8));
    board_cli.board.start_game();

    // TODO: track game stats
    // TODO: change game board to be x-axis A - H
    // TODO: change game board to be y-axis 1 - 8
    // TODO: add error handling for invalid characters & digits
    loop {
        println!("{}", board_cli);
        let mut s = String::new();
        print!("\nWhere do you want to shoot? ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        let coords: Vec<&str> = s.split(" ").collect();
        println!("\n{}\n", board_cli.shoot(coords[0], coords[1]));
    }
}

use battleship::{game::Game, game_trait::GameTrait, shoot_trait::ShootTrait};

#[test]
fn test_new() {
    let game = Game::new(8);
    assert_eq!(game.amt_of_turns, 0);
    assert_eq!(game.amt_of_hits, 0);
    assert_eq!(game.amt_of_misses, 0);
    assert_eq!(game.board.size, 8);
    assert_eq!(game.board.cells.len(), 0);
}

#[test]
fn test_start_game() {
    let mut game = Game::new(8);
    game.start_game();

    assert_eq!(game.amt_of_turns, 0);
    assert_eq!(game.amt_of_hits, 0);
    assert_eq!(game.amt_of_misses, 0);
    assert_eq!(game.board.size, 8);
    assert_eq!(game.board.cells.len(), 8);
}

#[test]
fn test_shoot() {
    let mut game = Game::new(8);
    game.add_destroyer();
    game.start_game();

    let shot_res = game.shoot(0, 0);
    if shot_res == 0 {
        assert!(game.amt_of_hits > 0);
        assert_eq!(game.amt_of_misses, 0);
    } else {
        assert_eq!(game.amt_of_hits, 0);
        assert!(game.amt_of_misses > 0);
    }
    assert!(game.amt_of_turns > 0);
}

#[test]
fn test_is_end() {
    let mut game = Game::new(8);
    game.add_destroyer();
    game.start_game();

    let mut ship_coords = Vec::new();
    for y in 0..game.board.cells.len() {
        for x in 0..game.board.cells[y].len() {
            let cell = &game.board.cells[y][x];
            if cell.cell_type == 1 {
                ship_coords.push([x as i32, y as i32]);
            }
        }
    }

    for ship_coord in ship_coords {
        game.shoot(ship_coord[0], ship_coord[1]);
        assert!(game.amt_of_hits > 0);
        assert!(game.amt_of_turns > 0);
    }

    assert!(game.is_end());
}

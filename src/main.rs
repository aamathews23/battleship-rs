use std::fmt;

use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut board = Board::new(8);
    board.start_game();

    println!("{}", board);

    board.shoot(0, 1);
    println!("{}", board);
    println!("{:?}", board.ships);

    board.shoot(1, 1);
    println!("{}", board);
    println!("{:?}", board.ships);

    board.shoot(1, 2);
    println!("{}", board);
    println!("{:?}", board.ships);

    board.shoot(1, 3);
    println!("{}", board);
    println!("{:?}", board.ships);

    board.shoot(1, 4);
    println!("{}", board);
    println!("{:?}", board.ships);
}

// TODO: Move cell to own file
#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Hit,
    Miss
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char = match self {
            Cell::Empty => " ",
            Cell::Hit => "X",
            Cell::Miss => "O"
        };
        write!(f, "{char}")
    }
}

// TODO: move random to own file
pub trait Random<T> {
    fn random(start: u32, end: u32) -> T;
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical
}

impl Random<Direction> for Direction {
    fn random(start: u32, end: u32) -> Direction {
        let choices = [start, end];
        let mut rng = thread_rng();
        let direction = match choices.choose(&mut rng) {
            Some(num) => *num,
            _ => 0
        };
        
        if direction == 0 {
            return Direction::Horizontal;
        } else {
            return Direction::Vertical;
        }
    }
}

// TODO: move ship size to own file
#[derive(Debug, Clone, Copy)]
enum ShipSize {
    Destroyer,
    Cruiser,
    Battleship
}

impl From<ShipSize> for u32 {
    fn from(value: ShipSize) -> Self {
        return match value {
            ShipSize::Destroyer => 2,
            ShipSize::Cruiser => 3,
            ShipSize::Battleship => 4
        }
    }
}

// TODO: move point to own file
#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point {
            x,
            y
        }
    }
}

impl Random<Point> for Point {
    fn random(start: u32, end: u32) -> Point {
        let range: Vec<u32> = (start..end).collect();
        let mut rng = thread_rng();
        let x = range.choose(&mut rng);
        let y = range.choose(&mut rng);
        Point {
            x: match x {
                Some(num) => *num,
                _ => 0
            },
            y: match y {
                Some(num) => *num,
                _ => 0
            }
        }
    }
}

// TODO: move ship to own file
#[derive(Debug, Clone, Copy)]
struct Ship {
    start: Point,
    end: Point,
    direction: Direction,
    size: ShipSize,
    health: u32
}

impl Ship {
    pub fn new(start: Point, end: Point, direction: Direction, size: ShipSize) -> Ship {
        Ship {
            start,
            end,
            direction,
            size,
            health: size.into()
        }
    }

    pub fn hit(&mut self) {
        self.health -= 1;
    }
}

// TODO: move shot outcome to own file
#[derive(Debug)]
enum ShotOutcome {
    Hit,
    Miss,
    TryAgain
}

// TODO: move shot result to own file
#[derive(Debug)]
struct ShotResult {
    outcome: ShotOutcome,
    hits_and_misses: Vec<Vec<Cell>>,
    ships: Vec<Ship>
}

impl ShotResult {
    pub fn new(outcome: ShotOutcome, hits_and_misses: Vec<Vec<Cell>>, ships: Vec<Ship>) -> ShotResult {
        ShotResult {
            outcome,
            hits_and_misses,
            ships
        }
    }
}

// TODO: move board to own file
#[derive(Debug)]
struct Board {
    size: u32,
    hits_and_misses: Vec<Vec<Cell>>,
    ships: Vec<Ship>
}

impl Board {
    pub fn new(size: u32) -> Board {
        let hits_and_misses: Vec<Vec<Cell>> = Vec::new();
        let ships: Vec<Ship> = Vec::new();
        Board {
            size,
            hits_and_misses,
            ships
        }
    }

    pub fn start_game(&mut self) {
        (0..self.size as usize).for_each(|i| {
            self.hits_and_misses.push(Vec::new());
            (0..self.size as usize).for_each(|_j| {
                self.hits_and_misses[i].push(Cell::Empty);
            });
        });
        self.ships.push(self.place_ship(ShipSize::Destroyer));
        self.ships.push(self.place_ship(ShipSize::Cruiser));
        self.ships.push(self.place_ship(ShipSize::Battleship));
    }

    pub fn shoot(&mut self, x: u32, y: u32) -> ShotResult {
        if x > self.size - 1 || y > self.size - 1 {
            return ShotResult::new(ShotOutcome::TryAgain, self.hits_and_misses.clone(), self.ships.clone());
        }

        for ship in &mut self.ships {
            // TODO: Move logic to ship struct, maybe a is_hit fn?
            match ship.direction {
                Direction::Horizontal => {
                    if y != ship.start.y {
                        self.hits_and_misses[x as usize][y as usize] = Cell::Miss;
                        return ShotResult::new(ShotOutcome::Miss, self.hits_and_misses.clone(), self.ships.clone());
                    }
                    let x_range = ship.start.x..ship.end.x;
                    println!("{:?}", x_range);
                    for i in x_range {
                        if x == i {
                            ship.hit();
                            break;
                        }
                    }
                    self.hits_and_misses[x as usize][y as usize] = Cell::Miss;
                    return ShotResult::new(ShotOutcome::Miss, self.hits_and_misses.clone(), self.ships.clone());
                },
                Direction::Vertical => {
                    if x != ship.start.x {
                        self.hits_and_misses[x as usize][y as usize] = Cell::Miss;
                        return ShotResult::new(ShotOutcome::Miss, self.hits_and_misses.clone(), self.ships.clone());
                    }
                    let y_range = ship.start.y..ship.end.y;
                    for i in y_range {
                        if y == i {
                            ship.hit();
                            break;
                        }
                    }
                    self.hits_and_misses[x as usize][y as usize] = Cell::Miss;
                    return ShotResult::new(ShotOutcome::Miss, self.hits_and_misses.clone(), self.ships.clone());
                }
            }
        }

        self.hits_and_misses[x as usize][y as usize] = Cell::Hit;

        return ShotResult::new(ShotOutcome::Hit, self.hits_and_misses.clone(), self.ships.clone());
    }

    fn valid_ship(&self, start: Point, end: Point) -> bool {
        if end.x > self.size - 1 || end.y > self.size - 1 {
            return false;
        }

        for ship in &self.ships {
            if start.x <= ship.start.x && end.x >= ship.start.x && start.y >= ship.start.y && start.y <= ship.end.y {
                return false;
            }
            if start.y <= ship.start.y && end.y >= ship.start.y && start.x >= ship.start.x && start.x <= ship.end.x {
                return false;
            }
        }

        true
    }

    /// TODO: fix bug with placing ships incorrectly on x and y axis
    fn place_ship(&self, ship_size: ShipSize) -> Ship {

        // TODO: Move logic to ship struct
        let mut direction = Direction::random(0, 1);
        let mut start = Point::random(0, self.size - 1);
        let mut end;
        let size: u32 = ship_size.into();
        match direction {
            Direction::Horizontal => {
                end = Point::new( start.x + size - 1, start.y);
            }
            _ => {
                end = Point::new(start.x, start.y + size - 1);
            }
        }

        while !self.valid_ship(start, end) {
            direction = Direction::random(0, 1);
            start = Point::random(0, self.size - 1);
            match direction {
                Direction::Horizontal => {
                    end = Point::new(start.x + size - 1, start.y);
                }
                _ => {
                    end = Point::new(start.x, start.y + size - 1);
                }
            }
        }

        Ship::new(start, end, direction, ship_size)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = "-----------------".to_owned();
        
        for row in &self.hits_and_misses {
            board.push_str("\n|");
            for cell in row {
                board.push_str(&cell.to_string());
                board.push_str("|");
            }
            board.push_str("\n-----------------");
        }
        write!(f, "{board}")
    }
}

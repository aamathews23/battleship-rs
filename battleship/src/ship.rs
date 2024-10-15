use crate::{
    direction::Direction,
    point::Point,
    random::Random
};

#[derive(Debug, Clone, Copy)]
pub enum ShipSize {
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

#[derive(Debug, Clone, Copy)]
pub struct Ship {
    pub start: Point,
    pub end: Point,
    pub direction: Direction,
    pub size: ShipSize,
    pub health: u32
}

impl Ship {
    pub fn new(start: u32, end: u32, size: ShipSize) -> Ship {
        let direction = Direction::random(0, 1);
        let size_addon: u32 = size.into();
        let s = Point::random(start, end - size_addon);
        let e;
        match direction {
            Direction::Horizontal => {
                e = Point::new( s.x + size_addon, s.y);
            }
            _ => {
                e = Point::new(s.x, s.y + size_addon);
            }
        }
        Ship {
            start: s,
            end: e,
            direction,
            size,
            health: size.into()
        }
    }

    pub fn place(&mut self, start: u32, end: u32) {
        self.direction = Direction::random(0, 1);
        let size_addon: u32 = self.size.into();
        self.start = Point::random(start, end - size_addon);
        match self.direction {
            Direction::Horizontal => {
                self.end = Point::new( self.start.x + size_addon, self.start.y);
            }
            _ => {
                self.end = Point::new(self.start.x, self.start.y + size_addon);
            }
        }
    }
}
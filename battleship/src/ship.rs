use crate::direction::Direction;

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

pub struct Ship {
    pub direction: Direction,
    pub size: ShipSize,
    pub health: u32
}

impl Ship {
    pub fn new(size: ShipSize) -> Ship {
        let direction = Direction::random(0, 1);
        Ship {
            direction,
            size,
            health: size.into()
        }
    }

    pub fn hit(&mut self) {
        self.health -= 1;
    }
}
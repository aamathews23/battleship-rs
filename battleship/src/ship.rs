use crate::direction::Direction;
use crate::random_generator::RandomGeneratorImpl;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShipSize {
    Destroyer,
    Cruiser,
    Battleship
}

impl From<ShipSize> for i32 {
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
    pub health: i32
}

impl Ship {
    pub fn new(size: ShipSize) -> Ship {
        let mut generator = RandomGeneratorImpl::new();
        let direction = Direction::random(&mut generator);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_size_destoryer_converts_to_2() {
        let ship_size = ShipSize::Destroyer;
        let ship_size_u32: i32 = ship_size.into();

        assert_eq!(ship_size_u32, 2);
    }

    #[test]
    fn test_ship_size_cruiser_converts_to_3() {
        let ship_size = ShipSize::Cruiser;
        let ship_size_u32: i32 = ship_size.into();

        assert_eq!(ship_size_u32, 3);
    }

    #[test]
    fn test_ship_size_battleship_converts_to_4() {
        let ship_size = ShipSize::Battleship;
        let ship_size_u32: i32 = ship_size.into();

        assert_eq!(ship_size_u32, 4);
    }

    #[test]
    fn test_ship_new() {
        let ship = Ship::new(ShipSize::Destroyer);

        assert_eq!(ship.health, 2);
        assert_eq!(ship.size, ShipSize::Destroyer);
    }

    #[test]
    fn test_ship_hit() {
        let mut ship = Ship::new(ShipSize::Destroyer);

        assert_eq!(ship.health, 2);
        ship.hit();
        assert_eq!(ship.health, 1);
    }
}

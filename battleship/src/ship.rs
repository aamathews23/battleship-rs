use crate::direction::Direction;
use crate::random_generator::RandomGeneratorImpl;

#[derive(Debug, PartialEq)]
pub struct Ship {
    pub direction: Direction,
    pub size: i32,
    pub health: i32
}

impl Ship {
    pub fn new(size: i32) -> Ship {
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
    fn test_ship_new() {
        let ship = Ship::new(2);

        assert_eq!(ship.health, 2);
        assert_eq!(ship.size, 2);
    }

    #[test]
    fn test_ship_hit() {
        let mut ship = Ship::new(2);

        assert_eq!(ship.health, 2);
        ship.hit();
        assert_eq!(ship.health, 1);
    }
}

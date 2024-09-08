use crate::random::Random;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
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
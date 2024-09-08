use crate::random::Random;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32
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
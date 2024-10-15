use std::fmt;
use battleship::shot::Shot;

pub struct ShotCli {
    shot: Shot
}

impl ShotCli {
    pub fn new(shot: Shot) -> ShotCli {
        ShotCli {
            shot
        }
    }
}

impl fmt::Display for ShotCli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char = match self.shot {
            Shot::Hit => "Hit!",
            Shot::Miss => "Miss...",
            Shot::TryAgain => "Try again."
        };
        write!(f, "{char}")
    }
}

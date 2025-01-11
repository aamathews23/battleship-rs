#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShootTraitResult {
    Miss = -1,
    Repeat = 0,
    Hit = 1,
    Sunk = 2
}

pub trait ShootTrait {
    fn shoot(&mut self, x: u32, y: u32) -> ShootTraitResult;
}

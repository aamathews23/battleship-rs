#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardCell {
    Miss = -1,
    Unknown = 0,
    Hit = 1,
    Ship = 2
}
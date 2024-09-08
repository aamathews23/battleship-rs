#[derive(Debug)]
pub struct ShotResult {
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
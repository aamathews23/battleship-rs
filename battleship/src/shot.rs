use crate::{
  cell::Cell,
  ship::Ship,
};

#[derive(Debug)]
pub enum Outcome {
    Hit,
    Miss,
    TryAgain
}

#[derive(Debug)]
pub struct Result {
    outcome: Outcome,
    hits_and_misses: Vec<Vec<Cell>>,
    ships: Vec<Ship>
}

impl Result {
    pub fn new(outcome: Outcome, hits_and_misses: Vec<Vec<Cell>>, ships: Vec<Ship>) -> Result {
        Result {
            outcome,
            hits_and_misses,
            ships
        }
    }
}
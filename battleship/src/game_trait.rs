use crate::random_generator::RandomGenerator;

pub trait GameTrait {
    fn start_game(&mut self, generator: &mut dyn RandomGenerator);
}

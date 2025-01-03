use mockall::automock;

#[automock]
pub trait RandomGeneratorTrait {
    fn generate(&mut self, start: i32, end: i32) -> i32;
}
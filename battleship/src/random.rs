pub trait Random<T> {
    fn random(start: u32, end: u32) -> T;
}
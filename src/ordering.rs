pub trait Ops<T> {
    /// Retrieved a list of characters ordered by it's recurrence
    fn get_ordered_characters(&self) -> Vec<T>;
}

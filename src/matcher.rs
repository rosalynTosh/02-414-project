pub trait Matcher<T> {
    fn find_indices_in(&self, text: &[T]) -> Vec<usize>;
}
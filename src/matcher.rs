pub trait Matcher<'p, T: 'p>: Sized {
    type IndexIterator<'i, 't>: Iterator<Item = usize> where Self: 'i, T: 't;

    fn from_pattern(pattern: &'p [T]) -> Option<Self>;

    fn find_indices_in<'i, 't>(&'i self, text: &'t [T]) -> Self::IndexIterator<'i, 't>;
}
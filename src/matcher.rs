pub trait HasIndexIterator<'p, T: 'p> {
    type IndexIterator<'t>: Iterator<Item = usize> where T: 't;
}

pub trait Matcher<'p, T: 'p>: HasIndexIterator<'p, T> {
    fn find_indices_in<'t>(&'p self, text: &'t [T]) -> Self::IndexIterator<'t>;
}
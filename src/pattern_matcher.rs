pub trait PatternMatcher<'p, T: 'p>: Sized {
    type IndexIterator<'i, 't>: Iterator<Item = usize> where Self: 'i, T: 't;

    fn from_pattern(pattern: &'p [T]) -> Option<Self>;

    fn find_indices_in<'i, 't>(&'i self, text: &'t [T]) -> Self::IndexIterator<'i, 't>;
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_matcher_case {
    ( $Matcher:ident, $pattern_str:expr, $text_str:expr, $indices:expr ) => {
        {
            let pattern = $pattern_str.chars().collect::<Vec<_>>();
            let text = $text_str.chars().collect::<Vec<_>>();

            match $Matcher::from_pattern(&pattern) {
                Some(matcher) => {
                    itertools::assert_equal(matcher.find_indices_in(&text), $indices.iter().copied());
                }
                None => {}
            }
        }
    }
}
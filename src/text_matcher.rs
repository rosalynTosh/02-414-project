pub trait TextMatcher<'t, T: 't>: Sized {
    type IndexIterator<'i, 'p>: Iterator<Item = usize> where Self: 'i, T: 'p;

    fn from_text(text: &'t [T]) -> Option<Self>;

    fn find_indices_of<'i, 'p>(&'i self, pattern: &'p [T]) -> Self::IndexIterator<'i, 'p>;
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_text_matcher_case {
    ( $Matcher:ident, $pattern_str:expr, $text_str:expr, $indices:expr ) => {
        {
            let pattern = $pattern_str.chars().collect::<Vec<_>>();
            let text = $text_str.chars().collect::<Vec<_>>();

            match $Matcher::from_text(&text) {
                Some(matcher) => {
                    itertools::assert_equal(matcher.find_indices_of(&pattern), $indices.iter().copied());
                }
                None => {}
            }
        }
    }
}

#[macro_export]
macro_rules! text_matcher_unit_tests {
    ( $Matcher:ident ) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::assert_text_matcher_case as assert_matcher_case;
            use crate::matcher_unit_tests;

            matcher_unit_tests!($Matcher);
        }
    }
}
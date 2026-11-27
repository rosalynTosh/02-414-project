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
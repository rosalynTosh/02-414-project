use crate::{matcher::Matcher, unit_test_matcher};

pub struct NaiveMatcher<'p, T: Eq> {
    pattern: &'p [T],
}

pub struct NaiveIndexIterator<'m, 'p, 't, T: Eq> {
    matcher: &'m NaiveMatcher<'p, T>,
    text: &'t [T],
    i: usize,
}

impl<'m, 'p, 't, T: Eq> Iterator for NaiveIndexIterator<'m, 'p, 't, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: while self.i + self.matcher.pattern.len() < self.text.len() + 1 {
            for k in 0..self.matcher.pattern.len() {
                if self.text[self.i + k] != self.matcher.pattern[k] {
                    self.i += 1;
                    continue 'outer;
                }
            }

            self.i += 1;
            return Some(self.i - 1);
        }

        None
    }
}

impl<'p, T: Eq> Matcher<'p, T> for NaiveMatcher<'p, T> {
    type IndexIterator<'i, 't> = NaiveIndexIterator<'i, 'p, 't, T> where Self: 'i, T: 't;

    fn from_pattern(pattern: &'p [T]) -> Option<NaiveMatcher<'p, T>> {
        Some(NaiveMatcher {
            pattern,
        })
    }

    fn find_indices_in<'i, 't>(&'i self, text: &'t [T]) -> Self::IndexIterator<'i, 't> {
        NaiveIndexIterator {
            matcher: self,
            text,
            i: 0,
        }
    }
}

unit_test_matcher!(NaiveMatcher);
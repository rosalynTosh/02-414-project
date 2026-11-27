use crate::{text_matcher::TextMatcher, text_matcher_unit_tests};

pub struct NaiveMatcher<'t, T: Eq> {
    text: &'t [T],
}

pub struct NaiveIndexIterator<'m, 'p, 't, T: Eq> {
    matcher: &'m NaiveMatcher<'t, T>,
    pattern: &'p [T],
    i: usize,
}

impl<'m, 'p, 't, T: Eq> Iterator for NaiveIndexIterator<'m, 'p, 't, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: while self.i + self.pattern.len() < self.matcher.text.len() + 1 {
            for k in 0..self.pattern.len() {
                if self.matcher.text[self.i + k] != self.pattern[k] {
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

impl<'t, T: Eq> TextMatcher<'t, T> for NaiveMatcher<'t, T> {
    type IndexIterator<'i, 'p> = NaiveIndexIterator<'i, 'p, 't, T> where Self: 'i, T: 'p;

    fn from_text(text: &'t [T]) -> Option<NaiveMatcher<'t, T>> {
        Some(NaiveMatcher {
            text,
        })
    }

    fn find_indices_of<'i, 'p>(&'i self, pattern: &'p [T]) -> Self::IndexIterator<'i, 'p> {
        NaiveIndexIterator {
            matcher: self,
            pattern,
            i: 0,
        }
    }
}

text_matcher_unit_tests!(NaiveMatcher);
use crate::matcher::{HasIndexIterator, Matcher};

pub struct NaiveMatcher<'p, T: Eq> {
    pattern: &'p [T],
}

impl<'p, T: Eq> NaiveMatcher<'p, T> {
    pub fn from_pattern(pattern: &'p [T]) -> NaiveMatcher<'p, T> {
        NaiveMatcher {
            pattern,
        }
    }
}

pub struct NaiveIndexIterator<'p, 't, T: Eq> {
    matcher: &'p NaiveMatcher<'p, T>,
    text: &'t [T],
    i: usize,
}

impl<'p, 't, T: Eq> Iterator for NaiveIndexIterator<'p, 't, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: while self.i < ((self.text.len() + 1) - self.matcher.pattern.len()) {
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

impl<'p, T: Eq + 'p> HasIndexIterator<'p, T> for NaiveMatcher<'p, T> {
    type IndexIterator<'t> = NaiveIndexIterator<'p, 't, T> where T: 't;
}

impl<'p, T: Eq> Matcher<'p, T> for NaiveMatcher<'p, T> {
    fn find_indices_in<'t>(&'p self, text: &'t [T]) -> Self::IndexIterator<'t> {
        NaiveIndexIterator {
            matcher: self,
            text,
            i: 0,
        }
    }
}
use crate::{pattern_matcher::PatternMatcher, pattern_matcher_unit_tests};

pub struct KmpMemoMatcher<'p, T: Eq> {
    pattern: &'p [T],
    memo: Vec<usize>,
}

pub struct KmpMemoIndexIterator<'m, 'p, 't, T: Eq> {
    matcher: &'m KmpMemoMatcher<'p, T>,
    text: &'t [T],
    i: usize,
    j: usize,
}

impl<'m, 'p, 't, T: Eq> Iterator for KmpMemoIndexIterator<'m, 'p, 't, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.text.len() {
            while self.j > 0 && self.text[self.i] != self.matcher.pattern[self.j] {
                self.j = self.matcher.memo[self.j];
            }

            if self.text[self.i] == self.matcher.pattern[self.j] {
                self.j += 1;
            }

            if self.j == self.matcher.pattern.len() {
                self.j = self.matcher.memo[self.j];
                self.i += 1;
                return Some(self.i - self.matcher.pattern.len());
            }

            self.i += 1;
        }

        None
    }
}

impl<'p, T: Eq + 'p> PatternMatcher<'p, T> for KmpMemoMatcher<'p, T> {
    type IndexIterator<'i, 't> = KmpMemoIndexIterator<'i, 'p, 't, T> where Self: 'i, T: 't;

    fn from_pattern(pattern: &'p [T]) -> Option<KmpMemoMatcher<'p, T>> {
        if pattern.len() == 0 {
            return None;
        }

        Some(KmpMemoMatcher {
            pattern,
            memo: {
                let mut memo: Vec<usize> = Vec::with_capacity(pattern.len() + 1);

                memo.push(0);
                memo.push(0);

                let mut j: usize = 0;
                for i in 1..pattern.len() {
                    while j > 0 && pattern[i] != pattern[j] {
                        j = memo[j];
                    }

                    if pattern[i] == pattern[j] {
                        j += 1;
                    }

                    memo.push(j);
                }

                memo
            },
        })
    }

    fn find_indices_in<'i, 't>(&'i self, text: &'t [T]) -> Self::IndexIterator<'i, 't> {
        KmpMemoIndexIterator {
            matcher: self,
            text,
            i: 0,
            j: 0,
        }
    }
}

pattern_matcher_unit_tests!(KmpMemoMatcher);
use crate::{pattern_matcher::PatternMatcher, pattern_matcher_unit_tests, pattern_matchers::z_algorithm};

pub struct KmpSpmMatcher<'p, T: Eq> {
    pattern: &'p [T],
    spm: Vec<usize>,
}

pub struct KmpSpmIndexIterator<'m, 'p, 't, T: Eq> {
    matcher: &'m KmpSpmMatcher<'p, T>,
    text: &'t [T],
    p: usize,
    c: usize,
}

impl<'m, 'p, 't, T: Eq> Iterator for KmpSpmIndexIterator<'m, 'p, 't, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.c + ((self.matcher.pattern.len() - 1) - self.p) < self.text.len() {
            while self.p < self.matcher.pattern.len() && self.matcher.pattern[self.p] == self.text[self.c] {
                self.p += 1;
                self.c += 1;
            }

            if self.p == self.matcher.pattern.len() {
                if self.p == 0 {
                    self.c += 1;
                    return Some((self.c - 1) - self.matcher.pattern.len());
                } else {
                    self.p = self.matcher.spm[self.p];
                    return Some(self.c - self.matcher.pattern.len());
                }
            } else {
                if self.p == 0 {
                    self.c += 1;
                } else {
                    self.p = self.matcher.spm[self.p];
                }
            }
        }

        None
    }
}

impl<'p, T: Eq + 'p> PatternMatcher<'p, T> for KmpSpmMatcher<'p, T> {
    type IndexIterator<'i, 't> = KmpSpmIndexIterator<'i, 'p, 't, T> where Self: 'i, T: 't;

    fn from_pattern(pattern: &'p [T]) -> Option<KmpSpmMatcher<'p, T>> {
        if pattern.len() == 0 {
            return None;
        }

        Some(KmpSpmMatcher {
            pattern,
            spm: {
                let z_array = z_algorithm::construct_z_array(pattern);
                let mut spm: Vec<usize> = vec![0; pattern.len() + 1];

                let mut i = pattern.len() - 1;
                while i >= 1 {
                    spm[i + z_array[i]] = z_array[i];
                    i -= 1;
                }

                spm
            },
        })
    }

    fn find_indices_in<'i, 't>(&'i self, text: &'t [T]) -> Self::IndexIterator<'i, 't> {
        KmpSpmIndexIterator {
            matcher: self,
            text,
            p: 0,
            c: 0,
        }
    }
}

pattern_matcher_unit_tests!(KmpSpmMatcher);
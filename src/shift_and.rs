use std::{collections::HashMap, hash::Hash};

use crate::{matcher::Matcher, unit_test_matcher};

pub struct ShiftAndMatcher<'p, T: Eq + Hash + Copy> {
    pattern: &'p [T],
    pattern_bitstrings: HashMap<T, u64>,
}

pub struct ShiftAndIndexIterator<'m, 'p, 't, T: Eq + Hash + Copy> {
    matcher: &'m ShiftAndMatcher<'p, T>,
    text: &'t [T],
    i: usize,
    curr_bitstring: u64,
}

impl<'m, 'p, 't, T: Eq + Hash + Copy> Iterator for ShiftAndIndexIterator<'m, 'p, 't, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.text.len() {
            self.curr_bitstring = (self.curr_bitstring << 1u64) | 1u64;
            self.curr_bitstring &= match self.matcher.pattern_bitstrings.get(&self.text[self.i]) {
                Some(bs) => *bs,
                None => 0u64,
            };

            if (self.curr_bitstring & (1u64 << ((self.matcher.pattern.len() - 1) as u64))) != 0u64 {
                self.i += 1;
                return Some(self.i - self.matcher.pattern.len());
            }

            self.i += 1;
        }

        None
    }
}

impl<'p, T: Eq + Hash + Copy> Matcher<'p, T> for ShiftAndMatcher<'p, T> {
    type IndexIterator<'i, 't> = ShiftAndIndexIterator<'i, 'p, 't, T> where Self: 'i, T: 't;

    fn from_pattern(pattern: &'p [T]) -> Option<ShiftAndMatcher<'p, T>> {
        if pattern.len() == 0 || pattern.len() > 64 {
            return None;
        }
        
        Some(ShiftAndMatcher {
            pattern,
            pattern_bitstrings: {
                let mut pattern_bitstrings: HashMap<T, u64> = HashMap::new();

                for i in 0..pattern.len() {
                    pattern_bitstrings.insert(pattern[i], match pattern_bitstrings.get(&pattern[i]) {
                        Some(bs) => bs | (1u64 << (i as u64)),
                        None => 1u64 << (i as u64),
                    });
                }

                pattern_bitstrings
            },
        })
    }

    fn find_indices_in<'i, 't>(&'i self, text: &'t [T]) -> Self::IndexIterator<'i, 't> {
        ShiftAndIndexIterator {
            matcher: self,
            text,
            i: 0,
            curr_bitstring: 0,
        }
    }
}

unit_test_matcher!(ShiftAndMatcher);
use std::{collections::HashMap, hash::Hash};

use crate::matcher::{HasIndexIterator, Matcher};

pub fn match_with_shift_and_64_bit<T: Copy + Eq + Hash>(big_string: &[T], substring: &[T]) -> Option<Vec<usize>> {
    if substring.len() > 64 {
        return None;
    }

    let mut pattern_bitstrings: HashMap<T, u64> = HashMap::new();

    for i in 0..substring.len() {
        pattern_bitstrings.insert(substring[i], match pattern_bitstrings.get(&substring[i]) {
            Some(bs) => bs | (1u64 << (i as u64)),
            None => 1u64 << (i as u64),
        });
    }

    let mut match_indices: Vec<usize> = Vec::new();

    let mut curr_bitstring: u64 = 0;

    for i in 0..big_string.len() {
        curr_bitstring = (curr_bitstring << 1u64) | 1u64;
        curr_bitstring &= match pattern_bitstrings.get(&big_string[i]) {
            Some(bs) => *bs,
            None => 0u64,
        };
        if (curr_bitstring & (1u64 << ((substring.len() - 1) as u64))) != 0u64 {
            match_indices.push((i + 1) - substring.len());
        }
    }
    
    Some(match_indices)
}

pub struct ShiftAndMatcher<'p, T: Eq + Hash + Copy> {
    pattern: &'p [T],
    pattern_bitstrings: HashMap<T, u64>,
}

impl<'p, T: Eq + Hash + Copy> ShiftAndMatcher<'p, T> {
    pub fn from_pattern(pattern: &'p [T]) -> Option<ShiftAndMatcher<'p, T>> {
        if pattern.len() > 64 {
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
}

pub struct ShiftAndIndexIterator<'p, 't, T: Eq + Hash + Copy> {
    matcher: &'p ShiftAndMatcher<'p, T>,
    text: &'t [T],
    i: usize,
    curr_bitstring: u64,
}

impl<'p, 't, T: Eq + Hash + Copy> Iterator for ShiftAndIndexIterator<'p, 't, T> {
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

impl<'p, T: Eq + Hash + Copy + 'p> HasIndexIterator<'p, T> for ShiftAndMatcher<'p, T> {
    type IndexIterator<'t> = ShiftAndIndexIterator<'p, 't, T> where T: 't;
}

impl<'p, T: Eq + Hash + Copy> Matcher<'p, T> for ShiftAndMatcher<'p, T> {
    fn find_indices_in<'t>(&'p self, text: &'t [T]) -> Self::IndexIterator<'t> {
        ShiftAndIndexIterator {
            matcher: self,
            text,
            i: 0,
            curr_bitstring: 0,
        }
    }
}
use std::{collections::HashMap, hash::Hash};

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
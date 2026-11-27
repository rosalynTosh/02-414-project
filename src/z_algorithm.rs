use crate::{matcher::Matcher, unit_test_matcher};

pub fn construct_z_array<T: Eq>(str: &[T]) -> Vec<usize> {
    let size = str.len();

    let mut z_array: Vec<usize> = Vec::with_capacity(size);

    if size == 0 {
        return z_array;
    }

    z_array.push(size);

    if size == 1 {
        return z_array;
    }

    for k in 0..(size - 1) {
        if str[k] != str[k + 1] {
            z_array.push(k);
            break;
        }
    }

    if z_array.len() == 1 {
        z_array.push(size - 1);
    }

    if size == 2 {
        return z_array;
    }

    let mut lo: usize = 1;
    let mut hi: usize = 1 + z_array[1];
    'outer: for i in 2..size {
        if i < hi {
            let offset_z = z_array[i - lo];

            if i + offset_z < hi {
                z_array.push(offset_z);
            } else {
                for k in (hi - i)..(size - i) {
                    if str[k] != str[k + i] {
                        z_array.push(k);
                        lo = i;
                        hi = i + k;
                        continue 'outer;
                    }
                }

                z_array.push(size - i);
                lo = i;
                hi = size;
            }
        } else {
            for k in 0..(size - i) {
                if str[k] != str[k + i] {
                    z_array.push(k);
                    lo = i;
                    hi = i + k;
                    continue 'outer;
                }
            }

            z_array.push(size - i);
            lo = i;
            hi = size;
        }
    }
    
    z_array
}

pub struct ZAlgorithmMatcher<'p, T: Eq> {
    pattern: &'p [T],
    trunc_z_array: Vec<usize>,
}

pub struct ZAlgorithmIndexIterator<'m, 'p, 't, T: Eq> {
    matcher: &'m ZAlgorithmMatcher<'p, T>,
    text: &'t [T],
    i: usize,
    lo: usize,
    hi: usize,
}

impl<'m, 'p, 't, T: Eq> Iterator for ZAlgorithmIndexIterator<'m, 'p, 't, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: while self.i < self.text.len() {
            if self.i < self.hi {
                let offset_z = self.matcher.trunc_z_array[(self.i - self.lo) - 1];

                if self.i + offset_z >= self.hi {
                    let mut k: usize = self.hi - self.i;
                    while k < self.matcher.pattern.len() && k < self.text.len() - self.i {
                        if self.matcher.pattern[k] != self.text[k + self.i] {
                            self.lo = self.i;
                            self.hi = self.i + k;
                            self.i += 1;
                            continue 'outer;
                        }

                        k += 1;
                    }

                    self.lo = self.i;
                    self.hi = self.i + k;
                    self.i += 1;
                    if k == self.matcher.pattern.len() {
                        return Some(self.lo);
                    }
                }
            } else {
                let mut k: usize = 0;
                while k < self.matcher.pattern.len() && k < self.text.len() - self.i {
                    if self.matcher.pattern[k] != self.text[k + self.i] {
                        self.lo = self.i;
                        self.hi = self.i + k;
                        self.i += 1;
                        continue 'outer;
                    }

                    k += 1;
                }

                self.lo = self.i;
                self.hi = self.i + k;
                self.i += 1;
                if k == self.matcher.pattern.len() {
                    return Some(self.lo);
                }
            }

            self.i += 1;
        }

        None
    }
}

impl<'p, T: Eq> Matcher<'p, T> for ZAlgorithmMatcher<'p, T> {
    type IndexIterator<'i, 't> = ZAlgorithmIndexIterator<'i, 'p, 't, T> where Self: 'i, T: 't;

    fn from_pattern(pattern: &'p [T]) -> Option<ZAlgorithmMatcher<'p, T>> {
        if pattern.len() == 0 {
            return None;
        }

        Some(ZAlgorithmMatcher {
            pattern,
            trunc_z_array: construct_z_array(pattern)[1..].to_owned(),
        })
    }

    fn find_indices_in<'i, 't>(&'i self, text: &'t [T]) -> Self::IndexIterator<'i, 't> {
        ZAlgorithmIndexIterator {
            matcher: self,
            text,
            i: 0,
            lo: 0,
            hi: 0,
        }
    }
}

unit_test_matcher!(ZAlgorithmMatcher);
use crate::{matcher::{HasIndexIterator, Matcher}, z_algorithm};

pub fn match_with_memo_kmp<T: Copy + Eq>(big_string: &[T], substring: &[T]) -> Vec<usize> {
    let memo: Vec<usize> = {
        let mut memo: Vec<usize> = Vec::with_capacity(substring.len() + 1);

        memo.push(0);
        memo.push(0);

        let mut j: usize = 0;
        for i in 1..substring.len() {
            while j > 0 && substring[i] != substring[j] {
                j = memo[j];
            }

            if substring[i] == substring[j] {
                j += 1;
            }

            memo.push(j);
        }

        memo
    };

    let mut match_indices: Vec<usize> = Vec::new();

    let mut j: usize = 0;
    for i in 0..big_string.len() {
        while j > 0 && big_string[i] != substring[j] {
            j = memo[j];
        }

        if big_string[i] == substring[j] {
            j += 1;
        }

        if j == substring.len() {
            match_indices.push((i + 1) - substring.len());
            j = memo[j];
        }
    }

    match_indices
}

pub fn match_with_spm_kmp<T: Copy + Eq>(big_string: &[T], substring: &[T]) -> Vec<usize> {
    let spm: Vec<usize> = {
        let z_array = z_algorithm::construct_z_array(substring);
        let mut spm: Vec<usize> = vec![0; substring.len() + 1];

        let mut i = substring.len() - 1;
        while i >= 1 {
            spm[i + z_array[i]] = z_array[i];
            i -= 1;
        }
        
        spm
    };

    let mut match_indices: Vec<usize> = Vec::new();

    let mut p: usize = 0;
    let mut c: usize = 0;
    while c + (substring.len() - p) < big_string.len() {
        while p < substring.len() && substring[p] == big_string[c] {
            p += 1;
            c += 1;
        }

        if p == substring.len() {
            match_indices.push(c - substring.len());
        }

        if p == 0 {
            c += 1;
        } else {
            p = spm[p];
        }
    }
    
    match_indices
}

pub struct KmpMemoMatcher<'p, T: Eq> {
    pattern: &'p [T],
    memo: Vec<usize>,
}

impl<'p, T: Eq> KmpMemoMatcher<'p, T> {
    pub fn from_pattern(pattern: &'p [T]) -> KmpMemoMatcher<'p, T> {
        KmpMemoMatcher {
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
        }
    }
}

pub struct KmpMemoIndexIterator<'p, 't, T: Eq> {
    matcher: &'p KmpMemoMatcher<'p, T>,
    text: &'t [T],
    i: usize,
    j: usize,
}

impl<'p, 't, T: Eq> Iterator for KmpMemoIndexIterator<'p, 't, T> {
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

impl<'p, T: Eq + 'p> HasIndexIterator<'p, T> for KmpMemoMatcher<'p, T> {
    type IndexIterator<'t> = KmpMemoIndexIterator<'p, 't, T> where T: 't;
}

impl<'p, T: Eq + 'p> Matcher<'p, T> for KmpMemoMatcher<'p, T> {
    fn find_indices_in<'t>(&'p self, text: &'t [T]) -> Self::IndexIterator<'t> {
        KmpMemoIndexIterator {
            matcher: self,
            text,
            i: 0,
            j: 0,
        }
    }
}

pub struct KmpSpmMatcher<'p, T: Eq> {
    pattern: &'p [T],
    spm: Vec<usize>,
}

impl<'p, T: Eq> KmpSpmMatcher<'p, T> {
    pub fn from_pattern(pattern: &'p [T]) -> KmpSpmMatcher<'p, T> {
        KmpSpmMatcher {
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
        }
    }
}

pub struct KmpSpmIndexIterator<'p, 't, T: Eq> {
    matcher: &'p KmpSpmMatcher<'p, T>,
    text: &'t [T],
    p: usize,
    c: usize,
}

impl<'p, 't, T: Eq> Iterator for KmpSpmIndexIterator<'p, 't, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.c + (self.matcher.pattern.len() - self.p) < self.text.len() {
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

impl<'p, T: Eq + 'p> HasIndexIterator<'p, T> for KmpSpmMatcher<'p, T> {
    type IndexIterator<'t> = KmpSpmIndexIterator<'p, 't, T> where T: 't;
}

impl<'p, T: Eq + 'p> Matcher<'p, T> for KmpSpmMatcher<'p, T> {
    fn find_indices_in<'t>(&'p self, text: &'t [T]) -> Self::IndexIterator<'t> {
        KmpSpmIndexIterator {
            matcher: self,
            text,
            p: 0,
            c: 0,
        }
    }
}
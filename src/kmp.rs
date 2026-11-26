use crate::z_algorithm;

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
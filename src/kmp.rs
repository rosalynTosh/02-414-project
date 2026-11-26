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
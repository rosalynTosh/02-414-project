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

pub fn match_with_z_algorithm<T: Copy + Eq>(big_string: &[T], substring: &[T], safe_delim: T) -> Vec<usize> {
    let mut concat_string: Vec<T> = Vec::with_capacity(big_string.len() + 1 + substring.len());
    concat_string.extend(substring);
    concat_string.push(safe_delim);
    concat_string.extend(big_string);

    let z_array = construct_z_array(&concat_string);

    let mut match_indices: Vec<usize> = Vec::new();

    for i in 0..big_string.len() {
        if z_array[i + substring.len() + 1] == substring.len() {
            match_indices.push(i);
        }
    }

    match_indices
}
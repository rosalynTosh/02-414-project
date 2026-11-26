use std::println;

pub fn construct_z_array_from_i32_string(str: &[i32]) -> Vec<usize> {
    let size = str.len();

    let mut z_array: Vec<usize> = Vec::with_capacity(size);

    z_array.push(size);

    for k in 0..(size - 1) {
        if str[k] != str[k + 1] {
            z_array.push(k);
            break;
        }
    }

    let mut lo: usize = 1;
    let mut hi: usize = 1 + z_array[1];
    'outer: for i in 2..size {
        println!("{} {:?}", i, z_array);

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

pub fn find_substring_with_z_algorithm(big_string: &[char], substring: &[char]) -> Vec<usize> {
    let mut concat_string: Vec<i32> = Vec::with_capacity(big_string.len() + 1 + substring.len());
    concat_string.extend(substring.iter().map(|ch| *ch as i32));
    concat_string.push(-1i32);
    concat_string.extend(big_string.iter().map(|ch| *ch as i32));

    let z_array = construct_z_array_from_i32_string(&concat_string);

    let mut match_indices: Vec<usize> = Vec::new();

    for i in 0..big_string.len() {
        if z_array[i + substring.len() + 1] == substring.len() {
            match_indices.push(i);
        }
    }

    match_indices
}
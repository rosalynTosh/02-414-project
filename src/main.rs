mod z_algorithm;
mod kmp;
mod shift_and;

fn main() {
    dbg!(z_algorithm::construct_z_array(&"aacataaaac".chars().collect::<Vec<_>>()));
    dbg!(z_algorithm::match_with_z_algorithm(&"catsanddogsdogwooddodogdodo".chars().collect::<Vec<_>>(), &"dog".chars().collect::<Vec<_>>(), '$'));
    dbg!(kmp::match_with_memo_kmp(&"catsanddogsdogwooddodogdodo".chars().collect::<Vec<_>>(), &"dog".chars().collect::<Vec<_>>()));
    dbg!(kmp::match_with_spm_kmp(&"catsanddogsdogwooddodogdodo".chars().collect::<Vec<_>>(), &"dog".chars().collect::<Vec<_>>()));
    dbg!(shift_and::match_with_shift_and_64_bit(&"catsanddogsdogwooddodogdodo".chars().collect::<Vec<_>>(), &"dog".chars().collect::<Vec<_>>()));

    dbg!(z_algorithm::match_with_z_algorithm(&"aacataaaac".chars().collect::<Vec<_>>(), &"aa".chars().collect::<Vec<_>>(), '$'));
    dbg!(kmp::match_with_memo_kmp(&"aacataaaac".chars().collect::<Vec<_>>(), &"aa".chars().collect::<Vec<_>>()));
    dbg!(kmp::match_with_spm_kmp(&"aacataaaac".chars().collect::<Vec<_>>(), &"aa".chars().collect::<Vec<_>>()));
    dbg!(shift_and::match_with_shift_and_64_bit(&"aacataaaac".chars().collect::<Vec<_>>(), &"aa".chars().collect::<Vec<_>>()));
}
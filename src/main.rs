mod naive;
mod z_algorithm;
mod kmp_memo;
mod kmp_spm;
mod shift_and;
mod matcher;
mod unit_test_matcher;

#[cfg(test)]
mod matcher_tests;

use matcher::Matcher;

fn main() {
    dbg!(z_algorithm::construct_z_array(&"aacataaaac".chars().collect::<Vec<_>>()));

    let dog_chars = "dog".chars().collect::<Vec<_>>();
    let dog_text_chars = "catsanddogsdogwooddodogdodo".chars().collect::<Vec<_>>();

    let z_algorithm_dog_matcher = z_algorithm::ZAlgorithmMatcher::from_pattern(&dog_chars).unwrap();
    dbg!(z_algorithm_dog_matcher.find_indices_in(&dog_text_chars).collect::<Vec<_>>());

    let naive_dog_matcher = naive::NaiveMatcher::from_pattern(&dog_chars).unwrap();
    dbg!(naive_dog_matcher.find_indices_in(&dog_text_chars).collect::<Vec<_>>());

    let kmp_memo_dog_matcher = kmp_memo::KmpMemoMatcher::from_pattern(&dog_chars).unwrap();
    dbg!(kmp_memo_dog_matcher.find_indices_in(&dog_text_chars).collect::<Vec<_>>());

    let kmp_spm_dog_matcher = kmp_spm::KmpSpmMatcher::from_pattern(&dog_chars).unwrap();
    dbg!(kmp_spm_dog_matcher.find_indices_in(&dog_text_chars).collect::<Vec<_>>());

    let shift_and_dog_matcher = shift_and::ShiftAndMatcher::from_pattern(&dog_chars).unwrap();
    dbg!(shift_and_dog_matcher.find_indices_in(&dog_text_chars).collect::<Vec<_>>());

    let aa_chars = "aa".chars().collect::<Vec<_>>();
    let aa_text_chars = "aacataaaac".chars().collect::<Vec<_>>();

    let naive_aa_matcher = naive::NaiveMatcher::from_pattern(&aa_chars).unwrap();
    dbg!(naive_aa_matcher.find_indices_in(&aa_text_chars).collect::<Vec<_>>());

    let z_algorithm_aa_matcher = z_algorithm::ZAlgorithmMatcher::from_pattern(&aa_chars).unwrap();
    dbg!(z_algorithm_aa_matcher.find_indices_in(&aa_text_chars).collect::<Vec<_>>());

    let kmp_memo_aa_matcher = kmp_memo::KmpMemoMatcher::from_pattern(&aa_chars).unwrap();
    dbg!(kmp_memo_aa_matcher.find_indices_in(&aa_text_chars).collect::<Vec<_>>());

    let kmp_spm_aa_matcher = kmp_spm::KmpSpmMatcher::from_pattern(&aa_chars).unwrap();
    dbg!(kmp_spm_aa_matcher.find_indices_in(&aa_text_chars).collect::<Vec<_>>());

    let shift_and_aa_matcher = shift_and::ShiftAndMatcher::from_pattern(&aa_chars).unwrap();
    dbg!(shift_and_aa_matcher.find_indices_in(&aa_text_chars).collect::<Vec<_>>());

    // let null_chars = "".chars().collect::<Vec<_>>();
    // let a_chars = "a".chars().collect::<Vec<_>>();

    // let naive_null_matcher = naive::NaiveMatcher::from_pattern(&null_chars).unwrap();
    // dbg!(naive_null_matcher.find_indices_in(&null_chars).collect::<Vec<_>>());
    // dbg!(naive_null_matcher.find_indices_in(&a_chars).collect::<Vec<_>>());

    // let z_algorithm_null_matcher = z_algorithm::ZAlgorithmMatcher::from_pattern(&null_chars).unwrap();
    // dbg!(z_algorithm_null_matcher.find_indices_in(&null_chars).collect::<Vec<_>>());
    // dbg!(z_algorithm_null_matcher.find_indices_in(&a_chars).collect::<Vec<_>>());

    // let kmp_memo_null_matcher = kmp_memo::KmpMemoMatcher::from_pattern(&null_chars).unwrap();
    // dbg!(kmp_memo_null_matcher.find_indices_in(&null_chars).collect::<Vec<_>>());
    // dbg!(kmp_memo_null_matcher.find_indices_in(&a_chars).collect::<Vec<_>>());

    // let kmp_spm_null_matcher = kmp_spm::KmpSpmMatcher::from_pattern(&null_chars).unwrap();
    // dbg!(kmp_spm_null_matcher.find_indices_in(&null_chars).collect::<Vec<_>>());
    // dbg!(kmp_spm_null_matcher.find_indices_in(&a_chars).collect::<Vec<_>>());

    // let shift_and_null_matcher = shift_and::ShiftAndMatcher::from_pattern(&null_chars).unwrap();
    // dbg!(shift_and_null_matcher.find_indices_in(&null_chars).collect::<Vec<_>>());
    // dbg!(shift_and_null_matcher.find_indices_in(&a_chars).collect::<Vec<_>>());

    let control_chars_str = {
        let mut control_chars_str = String::new();
        for i in 0x00..0x20 {
            control_chars_str.push(char::from_u32(i).unwrap());
        }
        control_chars_str.push('\x20');
        control_chars_str.push('\u{80}');
        control_chars_str.push('\u{81}');
        control_chars_str
    };
    let cc_chars = control_chars_str.chars().collect::<Vec<_>>();

    dbg!(&cc_chars);

    let naive_cc_matcher = naive::NaiveMatcher::from_pattern(&cc_chars).unwrap();
    dbg!(naive_cc_matcher.find_indices_in(&['\x00']).collect::<Vec<_>>());
}
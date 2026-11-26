mod naive;
mod z_algorithm;
mod kmp;
mod shift_and;
mod matcher;

use matcher::Matcher;

fn main() {
    dbg!(z_algorithm::construct_z_array(&"aacataaaac".chars().collect::<Vec<_>>()));

    let dog_chars = "dog".chars().collect::<Vec<_>>();
    let dog_text_chars = "catsanddogsdogwooddodogdodo".chars().collect::<Vec<_>>();

    let z_algorithm_dog_matcher = z_algorithm::ZAlgorithmMatcher::from_pattern(&dog_chars);
    dbg!(z_algorithm_dog_matcher.find_indices_in(&dog_text_chars).collect::<Vec<_>>());

    let naive_dog_matcher = naive::NaiveMatcher::from_pattern(&dog_chars);
    dbg!(naive_dog_matcher.find_indices_in(&dog_text_chars).collect::<Vec<_>>());

    let kmp_memo_dog_matcher = kmp::KmpMemoMatcher::from_pattern(&dog_chars);
    dbg!(kmp_memo_dog_matcher.find_indices_in(&dog_text_chars).collect::<Vec<_>>());

    let kmp_spm_dog_matcher = kmp::KmpSpmMatcher::from_pattern(&dog_chars);
    dbg!(kmp_spm_dog_matcher.find_indices_in(&dog_text_chars).collect::<Vec<_>>());

    let shift_and_dog_matcher = shift_and::ShiftAndMatcher::from_pattern(&dog_chars).unwrap();
    dbg!(shift_and_dog_matcher.find_indices_in(&dog_text_chars).collect::<Vec<_>>());

    let aa_chars = "aa".chars().collect::<Vec<_>>();
    let aa_text_chars = "aacataaaac".chars().collect::<Vec<_>>();

    let naive_aa_matcher = naive::NaiveMatcher::from_pattern(&aa_chars);
    dbg!(naive_aa_matcher.find_indices_in(&aa_text_chars).collect::<Vec<_>>());

    let z_algorithm_aa_matcher = z_algorithm::ZAlgorithmMatcher::from_pattern(&aa_chars);
    dbg!(z_algorithm_aa_matcher.find_indices_in(&aa_text_chars).collect::<Vec<_>>());

    let kmp_memo_aa_matcher = kmp::KmpMemoMatcher::from_pattern(&aa_chars);
    dbg!(kmp_memo_aa_matcher.find_indices_in(&aa_text_chars).collect::<Vec<_>>());

    let kmp_spm_aa_matcher = kmp::KmpSpmMatcher::from_pattern(&aa_chars);
    dbg!(kmp_spm_aa_matcher.find_indices_in(&aa_text_chars).collect::<Vec<_>>());

    let shift_and_aa_matcher = shift_and::ShiftAndMatcher::from_pattern(&aa_chars).unwrap();
    dbg!(shift_and_aa_matcher.find_indices_in(&aa_text_chars).collect::<Vec<_>>());
}
mod z_algorithm;

fn main() {
    dbg!(z_algorithm::find_substring_with_z_algorithm(&"catsanddogsdogwooddodogdodo".chars().collect::<Vec<_>>(), &"dog".chars().collect::<Vec<_>>()));
}
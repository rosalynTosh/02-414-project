mod z_algorithm;

fn main() {
    dbg!(z_algorithm::construct_z_array(&"aacataaaac".chars().collect::<Vec<_>>()));
    dbg!(z_algorithm::find_substring_with_z_algorithm("catsanddogsdogwooddodogdodo", "dog"));
}
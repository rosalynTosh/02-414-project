mod z_algorithm;

fn main() {
    dbg!(z_algorithm::construct_z_array(&"aacataaaac".chars().collect::<Vec<_>>()));
    dbg!(z_algorithm::match_with_z_algorithm("catsanddogsdogwooddodogdodo", "dog"));
}
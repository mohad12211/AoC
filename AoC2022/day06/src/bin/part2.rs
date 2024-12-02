use std::collections::HashSet;

fn are_elements_unique(slice: &[u8]) -> bool {
    let mut seen = HashSet::new();
    slice.iter().all(|item| seen.insert(item))
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let result = input.as_bytes().windows(14).position(are_elements_unique);
    println!("{}", result.unwrap() + 14);
}

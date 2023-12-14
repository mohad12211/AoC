use std::collections::HashSet;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let valid_count = input
        .lines()
        .filter_map(|l| {
            let mut set = HashSet::new();
            for word in l.split_whitespace() {
                if set.contains(word) {
                    return None;
                } else {
                    set.insert(word);
                }
            }
            Some(())
        })
        .count();
    println!("{valid_count}");
}

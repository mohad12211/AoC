use std::collections::HashSet;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let valid_count = input
        .lines()
        .filter_map(|l| {
            let mut set = HashSet::new();
            for word in l.split_whitespace() {
                let mut letters_map = [0; 26];
                for &b in word.as_bytes() {
                    letters_map[(b - b'a') as usize] += 1;
                }
                if set.contains(&letters_map) {
                    return None;
                } else {
                    set.insert(letters_map);
                }
            }
            Some(())
        })
        .count();
    println!("{valid_count}");
}

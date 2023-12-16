use std::collections::HashMap;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut map = HashMap::new();
    for line in input.lines() {
        let mut parts = line.splitn(4, ' ');
        let node = parts.next().unwrap();
        for child in parts.nth(2).unwrap_or_default().split(", ") {
            map.entry(child)
                .and_modify(|is_root| *is_root = false)
                .or_insert(false);
        }
        map.entry(node).or_insert(true);
    }

    let root = map.into_iter().find(|&(_, is_root)| is_root).unwrap().0;
    println!("{}", root);
}

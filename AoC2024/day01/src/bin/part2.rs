use std::collections::HashMap;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut map = HashMap::new();
    for line in input.lines() {
        let (_, right) = line.split_once("   ").unwrap();
        map.entry(right)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut result: i32 = 0;
    for line in input.lines() {
        let (left, _) = line.split_once("   ").unwrap();
        result += left.parse::<i32>().unwrap() * map.get(left).unwrap_or(&0);
    }
    println!("{result}");
}

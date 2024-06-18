use std::collections::HashMap;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let (two_count, three_count) = input
        .lines()
        .map(|l| {
            let mut map: HashMap<char, i32> = HashMap::new();
            l.chars().for_each(|c| {
                map.entry(c).and_modify(|n| *n += 1).or_insert(1);
            });
            (map.values().any(|&v| v == 2), map.values().any(|&v| v == 3))
        })
        .fold((0, 0), |(two_count, three_count), (has_two, has_three)| {
            (
                two_count + u64::from(has_two),
                three_count + u64::from(has_three),
            )
        });
    println!("{result}", result = two_count * three_count);
}

use std::collections::HashSet;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let freqs: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut seen_freqs = HashSet::new();
    let mut sum = 0;
    let first_repeated = freqs
        .into_iter()
        .cycle()
        .find_map(|c| {
            sum += c;
            seen_freqs.replace(sum)
        })
        .unwrap();
    println!("{first_repeated}");
}

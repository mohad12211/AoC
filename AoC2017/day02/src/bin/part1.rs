fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let sum: i64 = input
        .lines()
        .map(|l| {
            let mut nums: Vec<i64> = l.split_whitespace().map(|s| s.parse().unwrap()).collect();
            nums.sort();
            nums.last().unwrap() - nums.first().unwrap()
        })
        .sum();
    println!("{sum}");
}

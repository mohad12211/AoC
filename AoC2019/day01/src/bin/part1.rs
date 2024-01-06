fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let sum: i64 = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .map(|x| x / 3 - 2)
        .sum();
    println!("{sum}");
}

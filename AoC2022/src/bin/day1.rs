fn main() {
    let input = include_str!("../inputs/day1.txt");
    let mut totals: Vec<i32> = vec![];

    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            totals.push(total);
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap();
        }
    }

    totals.sort_by(|a, b| b.cmp(a));
    println!("{}", totals.first().unwrap());
    println!("{}", totals.iter().take(3).sum::<i32>());
}

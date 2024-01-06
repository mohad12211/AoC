fn get_fuel(fuel: i64) -> i64 {
    let next_fuel = fuel / 3 - 2;
    if next_fuel <= 0 {
        return 0;
    }
    return next_fuel + get_fuel(next_fuel);
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let sum: i64 = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .map(get_fuel)
        .sum();
    println!("{sum}");
}

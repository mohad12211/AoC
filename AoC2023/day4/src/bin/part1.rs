use std::str::FromStr;

#[derive(Debug)]
struct Card {
    numbers: Vec<i32>,
    winning: Vec<i32>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once(": ").ok_or("Invalid Card Format")?;
        let (winning, numbers) = s.split_once(" | ").ok_or("No separator between numbers")?;
        let winning: Vec<i32> = winning
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();
        let numbers: Vec<i32> = numbers
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();
        Ok(Self { numbers, winning })
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let sum: u32 = input
        .lines()
        .map(|l| Card::from_str(l).unwrap())
        .map(|c| c.winning.iter().filter(|x| c.numbers.contains(x)).count() as u32)
        .filter(|n| *n != 0)
        .map(|n| 2u32.pow(n - 1))
        .sum();
    println!("{sum}");
}

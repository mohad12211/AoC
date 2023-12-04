use std::str::FromStr;

#[derive(Debug)]
struct Card {
    n1: Vec<i32>,
    n2: Vec<i32>,
    copies: i32,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once(": ").ok_or("Invalid Card Format")?;
        let (n1, n2) = s.split_once(" | ").ok_or("No separator between numbers")?;
        let n1: Vec<i32> = n1
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();
        let n2: Vec<i32> = n2
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();
        Ok(Self { n1, n2, copies: 1 })
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut cards: Vec<Card> = input.lines().map(|l| l.parse().unwrap()).collect();
    for i in 0..cards.len() {
        let matches_count = cards[i]
            .n1
            .iter()
            .filter(|n1| cards[i].n2.contains(n1))
            .count();
        for j in i + 1..(i + 1) + matches_count {
            cards[j].copies += cards[i].copies;
        }
    }
    let sum: i32 = cards.iter().map(|c| c.copies).sum();
    println!("{sum}");
}

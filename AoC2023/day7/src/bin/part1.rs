use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    cards: Vec<u32>,
}

struct Round {
    bid: usize,
    hand: Hand,
}

impl Hand {
    fn get_type(&self) -> usize {
        self.cards
            .iter()
            .map(|c| self.cards.iter().filter(|&c1| c == c1).count())
            .sum()
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cards: s
                .chars()
                .map(|c| match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c.to_digit(10).expect("should be valid label"),
                })
                .collect(),
        })
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').ok_or("should have hand and bid")?;
        Ok(Self {
            hand: hand.parse().unwrap(),
            bid: bid.parse().map_err(|_| "big should be a number")?,
        })
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut rounds: Vec<_> = input.lines().map(|l| Round::from_str(l).unwrap()).collect();
    // sorty by type first, if equal, sort by cards (which are numbers)
    rounds.sort_by_key(|r| (r.hand.get_type(), r.hand.cards.clone()));
    let sum: usize = rounds
        .iter()
        .enumerate()
        .map(|(i, r)| ((i + 1) * r.bid))
        .sum();
    println!("{sum}");
}

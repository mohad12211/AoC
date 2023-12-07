use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    cards: Vec<u32>,
}

impl Hand {
    fn get_type(&self) -> usize {
        let non_j_occurrences: Vec<_> = self
            .cards
            .iter()
            .filter(|&&c| c != 1)
            .map(|c| self.cards.iter().filter(|&c1| c == c1).count())
            .collect();
        let j_count = self.cards.len() - non_j_occurrences.len();
        let non_j_max = non_j_occurrences.iter().max().unwrap_or(&0);
        let non_j_score: usize = non_j_occurrences.iter().sum();
        let j_score = (j_count * non_j_max) + (non_j_max + j_count) * j_count;
        return j_score + non_j_score;
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
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    'J' => 1,
                    _ => c.to_digit(10).expect("should be valid label"),
                })
                .collect(),
        })
    }
}

#[derive(Debug)]
struct Round {
    bid: usize,
    hand: Hand,
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').ok_or("should have hand and bid")?;
        Ok(Self {
            hand: hand.parse().unwrap(),
            bid: bid.parse().map_err(|_| "bid should be a number")?,
        })
    }
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut rounds: Vec<_> = input.lines().map(|l| Round::from_str(l).unwrap()).collect();
    rounds.sort_by_key(|r| (r.hand.get_type(), r.hand.cards.clone()));
    let sum: usize = rounds
        .iter()
        .enumerate()
        .map(|(i, r)| ((i + 1) * r.bid))
        .sum();
    println!("{sum}");
}

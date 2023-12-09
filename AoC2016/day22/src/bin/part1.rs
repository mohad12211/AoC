use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    used: usize,
    avail: usize,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        Ok(Self {
            used: parts[2]
                .strip_suffix('T')
                .and_then(|s| s.parse().ok())
                .ok_or("invalid used")?,
            avail: parts[3]
                .strip_suffix('T')
                .and_then(|s| s.parse().ok())
                .ok_or("invalid avail")?,
        })
    }
}

fn main() {
    let input = include_str!("input.txt");
    let nodes: Vec<_> = input
        .lines()
        .skip(2)
        .map(|l| Node::from_str(l).unwrap())
        .collect();

    let mut viable_pairs_count = 0;
    for a in &nodes {
        for b in &nodes {
            if a != b && a.used != 0 && b.avail >= a.used {
                viable_pairs_count += 1;
            }
        }
    }

    println!("{}", viable_pairs_count);
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let sum: i64 = input
        .lines()
        .map(|line| {
            let mut sequances = Vec::new();
            let original: Vec<i64> = line.split(' ').map(|c| c.parse().unwrap()).rev().collect();
            let mut seq = original;
            while seq.iter().any(|n| *n != seq[0]) {
                sequances.push(seq);
                seq = sequances
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect();
            }
            let mut last = seq[0];
            for seq in sequances.iter().rev() {
                last = seq.last().unwrap() + last;
            }
            last
        })
        .sum();
    println!("{sum}");
}

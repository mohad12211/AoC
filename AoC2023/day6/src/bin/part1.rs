fn main() {
    let mut lines = include_str!("input.txt").lines();
    // let mut lines = include_str!("input.test.txt").lines();
    let product: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .into_iter()
        .zip(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .filter_map(|n| n.parse().ok()),
        )
        .map(|(time, distance)| (1..time).filter(|t| (time - t) * t > distance).count())
        .product();
    println!("{product}");
}

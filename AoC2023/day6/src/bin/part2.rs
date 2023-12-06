fn main() {
    let mut lines = include_str!("input.txt").lines();
    // let mut lines = include_str!("input.test.txt").lines();
    let time: i64 = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance: i64 = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();

    let wins_count = (1..time).filter(|t| (time - t) * t > distance).count();
    println!("{wins_count}");
}

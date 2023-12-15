fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let sum: u64 = input.trim().split(',').map(hash).sum();
    println!("{sum}");
}

fn hash(s: &str) -> u64 {
    let mut current: u64 = 0;
    for &b in s.as_bytes() {
        current += b as u64;
        current *= 17;
        current %= 256;
    }
    current
}

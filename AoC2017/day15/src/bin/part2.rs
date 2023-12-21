fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let factors: [u64; 2] = [16807, 48271];
    let mut previous: Vec<u64> = input
        .lines()
        .map(|l| l.rsplit_once(' ').unwrap().1.parse().unwrap())
        .collect();

    let mut match_count = 0;
    for _ in 0..5_000_000 {
        previous[0] = (previous[0] * factors[0]) % 2147483647;
        previous[1] = (previous[1] * factors[1]) % 2147483647;
        while previous[0] % 4 != 0 {
            previous[0] = (previous[0] * factors[0]) % 2147483647;
        }
        while previous[1] % 8 != 0 {
            previous[1] = (previous[1] * factors[1]) % 2147483647;
        }

        if match16bit(previous[0], previous[1]) {
            match_count += 1;
        }
    }

    println!("{match_count}");
}

fn match16bit(a: u64, b: u64) -> bool {
    let mask = !(u64::MAX << 16);

    (a & mask) == (b & mask)
}

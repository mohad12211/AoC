fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let (start, end): (usize, usize) = input
        .trim()
        .split_once('-')
        .map(|(s, e)| (s.parse().unwrap(), e.parse().unwrap()))
        .unwrap();

    let mut count = 0;
    'outer: for mut i in start..=end {
        let mut prev = 10;
        let mut has_two_adj = false;
        while i != 0 {
            let n = i % 10;
            i /= 10;
            if n > prev {
                continue 'outer;
            }
            if n == prev {
                has_two_adj = true;
            }
            prev = n;
        }
        if has_two_adj {
            count += 1;
        }
    }
    println!("{count}");
}

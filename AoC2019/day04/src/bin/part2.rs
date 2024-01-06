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
        let mut adj_count = 0;
        while i != 0 {
            let n = i % 10;
            i /= 10;

            if n > prev {
                continue 'outer;
            }

            if n == prev {
                adj_count += 1;
            } else if adj_count == 1 {
                has_two_adj = true;
            } else {
                adj_count = 0;
            }

            prev = n;
        }
        if has_two_adj || adj_count == 1 {
            count += 1;
        }
    }
    println!("{count}");
}

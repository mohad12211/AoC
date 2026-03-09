fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut sum: usize = 0;
    for range in input.trim().split(',') {
        let (start, end): (usize, usize) = range
            .split_once('-')
            .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
            .unwrap();

        for i in start..=end {
            let i_string = i.to_string();
            let i_len = i_string.len();

            if i_len % 2 == 1 {
                continue;
            }

            if i_string[..i_len / 2] == i_string[i_len / 2..] {
                sum += i;
            }
        }
    }

    println!("{sum}");
}

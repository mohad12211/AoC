fn has_repeating(num: usize) -> bool {
    let str = num.to_string();
    let len = str.len();

    'outer: for count in 1..=(len / 2) {
        if len % count != 0 {
            continue;
        }

        let parts = len / count;

        for part in 0..parts - 1 {
            if str[(count * part)..(count * (part + 1))]
                != str[(count * (part + 1))..(count * (part + 2))]
            {
                continue 'outer;
            }
        }

        return true;
    }

    return false;
}

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
            if has_repeating(i) {
                sum += i;
            }
        }
    }

    println!("{sum}");
}

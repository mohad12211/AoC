fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");
    let mut ranges: Vec<(u32, u32)> = input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    ranges.sort();
    let mut first_allowed = 0;
    for i in 0..ranges.len() {
        let &(min, max) = ranges.get(i).unwrap();
        if max == first_allowed {
            first_allowed += 1;
        } else if min <= first_allowed && max > first_allowed {
            first_allowed = max + 1;
        }
    }
    println!("{first_allowed}");
}

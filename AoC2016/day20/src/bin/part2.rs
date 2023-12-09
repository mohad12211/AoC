use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");
    let mut ranges: Vec<(u32, u32)> = input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    ranges.sort();
    let mut stack = VecDeque::new();
    stack.push_front(ranges[0]);
    for i in 1..ranges.len() {
        let &(start, end) = ranges.get(i).unwrap();
        let &(prev_start, prev_end) = stack.front().unwrap();

        if prev_end < start {
            stack.push_front((start, end))
        } else if prev_end < end {
            stack.pop_front();
            stack.push_front((prev_start, end));
        }
    }

    let mut blocked_count: u32 = 0;
    for (start, end) in stack {
        blocked_count += (start..=end).count() as u32;
    }
    let allowd_count = u32::MAX - blocked_count + 1;
    println!("{allowd_count}");
}

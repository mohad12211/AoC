use std::collections::HashSet;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let points: HashSet<(usize, usize)> = input
        .lines()
        .map(|l| l.split_once(", ").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let min_x = *points.iter().map(|(x, _)| x).min().unwrap() - 1;
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap() + 1;

    let min_y = *points.iter().map(|(_, y)| y).min().unwrap() - 1;
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap() + 1;

    let mut count = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if points
                .iter()
                .map(|&(px, py)| x.abs_diff(px) + y.abs_diff(py))
                .sum::<usize>()
                < 10000
            {
                count += 1;
            }
        }
    }

    println!("{count}");
}

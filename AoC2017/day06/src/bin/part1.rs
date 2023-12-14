use std::collections::HashSet;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let banks: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let banks_count = banks.len();

    let mut set = HashSet::new();
    let mut current = banks.clone();
    let mut counter = 0;
    while !set.contains(&current) {
        set.insert(current.clone());
        counter += 1;
        let (index, max) = find_max(&current);
        current[index] = 0;
        for i in 1..=max {
            current[(index + i) % banks_count] += 1;
        }
    }
    println!("{counter}");
}

fn find_max(banks: &Vec<usize>) -> (usize, usize) {
    let max = banks.iter().max().unwrap();
    let i = banks.iter().position(|x| x == max).unwrap();
    (i, *max)
}

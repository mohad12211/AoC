fn main() {
    let input = include_str!("../inputs/day4.txt");

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let contained = input
        .lines()
        .map(|l| {
            l.split(|c| c == ',' || c == '-')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|x| (x[0] - x[2]) * (x[1] - x[3]) <= 0)
        .count();
    println!("{contained}");
}

fn part2(input: &str) {
    let overlap = input
        .lines()
        .map(|l| {
            l.split(|c| c == ',' || c == '-')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|x| ((x[1] - x[2]) >= 0) != (x[0] - x[3] > 0))
        .count();
    println!("{overlap}");
}

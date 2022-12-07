fn main() {
    let input = include_str!("../inputs/day3.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut sum = 0;
    for l in input.lines() {
        let prio = l[0..l.len() / 2]
            .chars()
            .filter(|k| l[l.len() / 2..].contains(*k))
            .collect::<Vec<char>>()[0] as u8;
        sum += match prio.is_ascii_uppercase() {
            true => (prio - b'A') as i32 + 1 + 26,
            false => (prio - b'a') as i32 + 1,
        };
    }
    println!("{}", sum);
}
fn part2(input: &str) {
    let mut sum = 0;
    for l in input.lines().collect::<Vec<&str>>().chunks(3) {
        let prio = l[0]
            .chars()
            .filter(|k| l.iter().all(|s| s.contains(*k)))
            .collect::<Vec<char>>()[0] as u8;
        sum += match prio.is_ascii_uppercase() {
            true => (prio - b'A') as i32 + 1 + 26,
            false => (prio - b'a') as i32 + 1,
        };
    }
    println!("{}", sum);
}

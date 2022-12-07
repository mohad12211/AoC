fn main() {
    let input = include_str!("../inputs/day2.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut score = 0;
    for i in input.lines() {
        let choices = i.as_bytes();
        let opp = choices[0] - b'A';
        let me = choices[2] - b'X';
        if (opp + 1) % 3 == me {
            score += 6;
        } else if opp == me {
            score += 3;
        }
        score += me as i32 + 1;
    }
    println!("{}", score);
}

fn part2(input: &str) {
    let mut score = 0;
    for i in input.lines() {
        let choices = i.as_bytes();
        let opp = choices[0] - b'A';
        let me = choices[2] - b'X';
        score += (me as i32) * 3;
        score += ((me as i32) - 1 + (opp as i32)).rem_euclid(3) + 1;
    }
    println!("{}", score);
}

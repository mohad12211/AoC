fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut dial = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let dir = &line[..1];
        let num: i64 = line[1..].parse().unwrap();

        zeros += num / 100;
        let remainder = num % 100;

        match dir {
            "L" => {
                let dist_to_zero = if dial == 0 { 100 } else { dial };
                if remainder >= dist_to_zero {
                    zeros += 1;
                }
                dial = (dial - remainder).rem_euclid(100);
            }
            "R" => {
                let dist_to_zero = if dial == 0 { 100 } else { 100 - dial };
                if remainder >= dist_to_zero {
                    zeros += 1;
                }
                dial = (dial + remainder) % 100;
            }
            _ => unreachable!("Invalid Direction"),
        }
    }

    println!("{zeros}");
}

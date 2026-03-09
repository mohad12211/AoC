fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut dial = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let dir = &line[..1];
        let num: i64 = line[1..].parse().unwrap();

        match dir {
            "L" => {
                dial = (dial - num).rem_euclid(100);
            }
            "R" => dial = (dial + num).rem_euclid(100),
            _ => unreachable!("Invalid Direction"),
        }

        if dial == 0 {
            zeros += 1;
        }
    }

    println!("{zeros}");
}

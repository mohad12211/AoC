use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    let sum: i32 = input
        .lines()
        .map(|line| {
            let first_re =
                Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
            let last_re =
                Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
            let parse = |x| match x {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => 0,
            };
            let first = &first_re.captures(line).unwrap()[1];
            let last = &last_re.captures(line).unwrap()[1];

            let first = first.parse().unwrap_or_else(|_| parse(first));
            let last = last.parse().unwrap_or_else(|_| parse(last));

            first * 10 + last
        })
        .sum();
    println!("{sum}");
}

fn main() {
    let input = include_str!("../../input.txt");

    let sum: u32 = input
        .lines()
        .map(|line| {
            let x: Vec<char> = line.chars().filter(|x| x.is_digit(10)).collect();
            x.first().unwrap().to_digit(10).unwrap() * 10 + x.last().unwrap().to_digit(10).unwrap()
        })
        .sum();
    println!("{sum}");
}

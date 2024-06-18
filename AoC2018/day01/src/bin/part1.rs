fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let result: i32 = input.lines().map(|l| l.parse::<i32>().unwrap()).sum();
    println!("{result}");
}

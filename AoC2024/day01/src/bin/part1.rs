fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let (mut first, mut second): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            l.split_once("   ")
                .map(|(first, second)| {
                    (
                        first.parse::<i32>().unwrap(),
                        second.parse::<i32>().unwrap(),
                    )
                })
                .unwrap()
        })
        .unzip();
    first.sort();
    second.sort();
    let result: u32 = first
        .into_iter()
        .zip(second.into_iter())
        .map(|(first, second)| first.abs_diff(second))
        .sum();
    println!("{result}");
}

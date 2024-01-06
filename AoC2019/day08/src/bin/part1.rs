fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let digits: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let layer_with_fewest_zero = digits
        .chunks(25 * 6)
        .min_by_key(|l| l.iter().filter(|&&d| d == 0).count())
        .unwrap();
    let product = layer_with_fewest_zero.iter().filter(|&&d| d == 1).count()
        * layer_with_fewest_zero.iter().filter(|&&d| d == 2).count();
    println!("{product}");
}

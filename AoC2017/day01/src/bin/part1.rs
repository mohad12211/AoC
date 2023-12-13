fn main() {
    let input = include_str!("input.txt");
    let nums: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut sum = 0;
    for (i, &num) in nums.iter().enumerate() {
        if nums[(i + 1) % nums.len()] == num {
            sum += num;
        }
    }

    println!("{sum}");
}

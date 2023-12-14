fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let sum: i64 = input
        .lines()
        .map(|l| {
            let nums: Vec<i64> = l.split_whitespace().map(|s| s.parse().unwrap()).collect();
            for i in 0..nums.len() {
                for j in 0..nums.len() {
                    if i != j && nums[i] % nums[j] == 0 {
                        return (nums[i] / nums[j]) as i64;
                    }
                }
            }
            unreachable!()
        })
        .sum();
    println!("{sum}");
}

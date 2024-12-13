fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut nums: Vec<u64> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let blinks = 25;

    for _ in 0..blinks {
        let mut index = 0;
        let len = nums.len();
        loop {
            let n = nums[index];
            if n == 0 {
                nums[index] = 1;
            } else if count_digits(n) % 2 == 0 {
                let (left, right) = split_number(n);
                nums.push(right);
                nums[index] = left;
            } else {
                nums[index] = n * 2024;
            }
            index += 1;
            if index >= len {
                break;
            }
        }
    }

    println!("{}", nums.len());
}

fn count_digits(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    let mut number = n;
    while number > 0 {
        count += 1;
        number /= 10;
    }
    count
}

fn split_number(n: u64) -> (u64, u64) {
    let digits = count_digits(n);

    let half = digits / 2;
    let divisor = 10u64.pow(half);
    let left = n / divisor;
    let right = n % divisor;

    (left, right)
}

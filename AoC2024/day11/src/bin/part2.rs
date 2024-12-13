use std::collections::HashMap;

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let blinks = 75;
    let mut cache = HashMap::new();
    let result: usize = input
        .trim()
        .split(' ')
        .map(|s| count(s.parse().unwrap(), blinks, &mut cache))
        .sum();

    println!("{}", result);
}

fn count(stone: u64, steps: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if let Some(&cached) = cache.get(&(stone, steps)) {
        return cached;
    }
    if steps == 0 {
        return 1;
    }

    if stone == 0 {
        return count(1, steps - 1, cache);
    }

    if count_digits(stone) % 2 == 0 {
        let (left, right) = split_number(stone);
        let result = count(left, steps - 1, cache) + count(right, steps - 1, cache);
        cache.insert((stone, steps), result);
        return result;
    }

    let result = count(stone * 2024, steps - 1, cache);
    cache.insert((stone, steps), result);
    return result;
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

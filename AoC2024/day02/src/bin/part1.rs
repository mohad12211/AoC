fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut total = 0;
    for line in input.lines() {
        let result = line
            .split(' ')
            .collect::<Vec<_>>()
            .windows(2)
            .map(|a| a[0].parse::<i32>().unwrap() - a[1].parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if (result.iter().all(|x| *x > 0) || result.iter().all(|x| *x < 0))
            && result.iter().all(|x| x.abs() >= 1 && x.abs() <= 3)
        {
            total += 1;
        }
    }
    println!("{total}");
}

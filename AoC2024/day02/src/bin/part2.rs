fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut total = 0;
    for line in input.lines() {
        let v = line
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        if is_safe(&v) {
            total += 1;
        } else {
            for i in 0..v.len() {
                let mut x = v.clone();
                x.remove(i);
                if is_safe(&x) {
                    total += 1;
                    break;
                }
            }
        }
    }
    println!("{total}");
}

fn is_safe(line: &[i32]) -> bool {
    let result = line.windows(2).map(|a| a[0] - a[1]).collect::<Vec<_>>();
    (result.iter().all(|x| *x > 0) || result.iter().all(|x| *x < 0))
        && result.iter().all(|x| x.abs() >= 1 && x.abs() <= 3)
}

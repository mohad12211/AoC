fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut sum: usize = 0;

    for line in input.lines() {
        let (index, left) = line[0..line.len() - 1]
            .bytes()
            .enumerate()
            .rev()
            .max_by_key(|&(_, val)| val)
            .unwrap();

        let left = left as char;
        let right = line[(index + 1)..].bytes().max().unwrap() as char;

        sum += format!("{left}{right}").parse::<usize>().unwrap();
    }

    println!("{sum}")
}

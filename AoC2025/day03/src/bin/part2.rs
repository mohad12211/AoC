fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let mut sum: usize = 0;

    for line in input.lines() {
        let mut digits = Vec::with_capacity(12);

        let mut preivous_index = None;
        for i in (0..=11).rev() {
            let cut = &line[preivous_index.map_or(0, |i| i + 1)..line.len() - i];

            let (index, digit) = cut
                .bytes()
                .enumerate()
                .rev()
                .max_by_key(|&(_, val)| val)
                .unwrap();

            digits.push((digit as char).to_string());

            preivous_index = Some(index + preivous_index.map_or(0, |i| i + 1));
        }

        sum += digits.join("").parse::<usize>().unwrap();
    }

    println!("{sum}")
}

fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    for x in input.lines() {
        for y in input.lines() {
            let diffs: Vec<_> = x
                .chars()
                .zip(y.chars())
                .enumerate()
                .filter(|(_, (c1, c2))| c1 != c2)
                .collect();
            if diffs.len() == 1 {
                let (diff_index, _) = diffs[0];
                let mut result = x.to_string();
                result.replace_range(diff_index..=diff_index, "");
                println!("{result}");
                return;
            }
        }
    }
}

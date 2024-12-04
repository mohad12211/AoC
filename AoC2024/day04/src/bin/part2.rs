fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let rows = input.lines().count() as i32;
    let columns = input.find('\n').unwrap() as i32;
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let word = "MAS".as_bytes();
    let word_reveresed = word.iter().copied().rev().collect::<Vec<u8>>();
    let cross1 = [(0, 0), (1, 1), (2, 2)];
    let cross2 = [(2, 0), (1, 1), (0, 2)];
    let mut count = 0;

    fn check_pattern(
        grid: &[&[u8]],
        row: i32,
        col: i32,
        word: &[u8],
        deltas: &[(i32, i32)],
    ) -> bool {
        deltas
            .iter()
            .copied()
            .map(|(dr, dc)| (row + dr, col + dc))
            .zip(word.iter())
            .all(|((row, col), letter)| {
                let (Some(row), Some(col)) = (usize::try_from(row).ok(), usize::try_from(col).ok())
                else {
                    return false;
                };
                grid.get(col).map(|line| line.get(row)) == Some(Some(letter))
            })
    }

    for row in 0..columns {
        for col in 0..rows {
            if (check_pattern(&grid, row, col, word, &cross1)
                || check_pattern(&grid, row, col, &word_reveresed, &cross1))
                && (check_pattern(&grid, row, col, word, &cross2)
                    || check_pattern(&grid, row, col, &word_reveresed, &cross2))
            {
                count += 1;
            }
        }
    }
    println!("{count}");
}

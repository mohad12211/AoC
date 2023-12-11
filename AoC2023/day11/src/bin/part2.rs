fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");
    let mut galaxies = Vec::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push((col, row));
            }
        }
    }

    for col in (0..width).rev() {
        let is_col_empty = galaxies.iter().all(|(pcol, _)| col != *pcol);
        if is_col_empty {
            galaxies
                .iter_mut()
                .filter(|(pcol, _)| *pcol > col)
                .for_each(|(pcol, _)| *pcol += 1_000_000 - 1);
        }
    }

    for row in (0..height).rev() {
        let is_row_empty = galaxies.iter().all(|(_, prow)| row != *prow);
        if is_row_empty {
            galaxies
                .iter_mut()
                .filter(|(_, prow)| *prow > row)
                .for_each(|(_, prow)| *prow += 1_000_000 - 1);
        }
    }

    let mut pairs_paths_sum: i64 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (c1, r1) = galaxies[i];
            let (c2, r2) = galaxies[j];
            pairs_paths_sum += (r2 as i64 - r1 as i64).abs() + (c2 as i64 - c1 as i64).abs();
        }
    }

    println!("{pairs_paths_sum}");
}

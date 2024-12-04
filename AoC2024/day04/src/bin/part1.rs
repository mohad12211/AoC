fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");

    let rows = input.lines().count() as i32;
    let columns = input.find('\n').unwrap() as i32;

    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let word = "XMAS".as_bytes();

    let mut count = 0;
    for row in 0..columns {
        for col in 0..rows {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if (0..word.len() as i32)
                        .map(|k| (row + dr * k, col + dc * k))
                        .zip(word.iter())
                        .all(|((row, col), letter)| {
                            let (Some(row), Some(col)) =
                                (usize::try_from(row).ok(), usize::try_from(col).ok())
                            else {
                                return false;
                            };

                            grid.get(col).map(|line| line.get(row)) == Some(Some(letter))
                        })
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{count}");
}

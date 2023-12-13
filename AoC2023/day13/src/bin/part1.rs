fn reflects_horizontally(grid: &Vec<Vec<char>>, (row1, row2): (usize, usize)) -> bool {
    for col in 0..grid[0].len() {
        if grid[row1][col] != grid[row2][col] {
            return false;
        }
    }

    if row1 > 0 && row2 + 1 < grid.len() {
        reflects_horizontally(grid, (row1 - 1, row2 + 1))
    } else {
        true
    }
}

fn reflects_vertically(grid: &Vec<Vec<char>>, (col1, col2): (usize, usize)) -> bool {
    for row in 0..grid.len() {
        if grid[row][col1] != grid[row][col2] {
            return false;
        }
    }

    if col1 > 0 && col2 + 1 < grid[0].len() {
        reflects_vertically(grid, (col1 - 1, col2 + 1))
    } else {
        true
    }
}

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");
    let sum: i64 = input
        .split("\n\n")
        .map(|grid| grid.lines().map(|l| l.chars().collect()).collect())
        .map(|grid: Vec<Vec<char>>| {
            for row in 0..grid.len() - 1 {
                if reflects_horizontally(&grid, (row, row + 1)) {
                    return (row as i64 + 1) * 100;
                }
            }
            for col in 0..grid[0].len() - 1 {
                if reflects_vertically(&grid, (col, col + 1)) {
                    return col as i64 + 1;
                }
            }
            unreachable!()
        })
        .sum();
    println!("{sum}");
}

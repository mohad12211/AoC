use std::collections::HashMap;

fn cycle(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'O' {
                let mut current_row = row;
                while current_row > 0 && grid[current_row - 1][col] == '.' {
                    grid[current_row][col] = '.';
                    grid[current_row - 1][col] = 'O';
                    current_row -= 1;
                }
            }
        }
    }

    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            if grid[row][col] == 'O' {
                let mut current_col = col;
                while current_col > 0 && grid[row][current_col - 1] == '.' {
                    grid[row][current_col] = '.';
                    grid[row][current_col - 1] = 'O';
                    current_col -= 1;
                }
            }
        }
    }

    for row in (0..grid.len()).rev() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'O' {
                let mut current_row = row;
                while current_row + 1 < grid.len() && grid[current_row + 1][col] == '.' {
                    grid[current_row][col] = '.';
                    grid[current_row + 1][col] = 'O';
                    current_row += 1;
                }
            }
        }
    }

    for col in (0..grid[0].len()).rev() {
        for row in 0..grid.len() {
            if grid[row][col] == 'O' {
                let mut current_col = col;
                while current_col + 1 < grid[0].len() && grid[row][current_col + 1] == '.' {
                    grid[row][current_col] = '.';
                    grid[row][current_col + 1] = 'O';
                    current_col += 1;
                }
            }
        }
    }
}

fn find_residue_period(mut grid: Vec<Vec<char>>) -> (usize, usize) {
    let mut map = HashMap::new();
    for i in 0.. {
        if let Some(&prev) = map.get(&grid) {
            return (prev, (i - prev));
        } else {
            map.insert(grid.clone(), i);
        }
        cycle(&mut grid);
    }
    unreachable!()
}

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("input.test.txt");
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let (residue, period) = find_residue_period(grid.clone());
    let cycles_to_do = ((1_000_000_000 - residue) % period) + residue;
    for _ in 0..cycles_to_do {
        cycle(&mut grid);
    }

    let mut sum = 0;
    let row_count = grid.len();
    for (i, row) in grid.iter().enumerate() {
        sum += (row_count - i) * row.iter().filter(|&&c| c == 'O').count();
    }
    println!("{sum}");
}

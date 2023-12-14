fn main() {
    // let input = include_str!("input.test.txt");
    let input = include_str!("input.txt");
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let row_count = grid.len();
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

    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        sum += (row_count - i) * row.iter().filter(|&&c| c == 'O').count();
    }
    println!("{sum}");
}

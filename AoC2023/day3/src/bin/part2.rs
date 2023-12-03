use std::collections::HashMap;

fn main() {
    // let input = include_str!("./input.test.txt");
    let input = include_str!("./input.txt");
    let lines: Vec<_> = input.lines().collect();
    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;
    let grid: Vec<_> = input.chars().filter(|x| *x != '\n').collect();
    let mut current_number = 0;
    let mut is_current_number_valid = false;
    let mut gear_map: HashMap<i32, Vec<_>> = HashMap::new();
    let mut gear_index = 0;
    for row in 0..rows {
        for col in 0..cols {
            let c = grid[(rows * row + col) as usize];
            if c.is_ascii_digit() {
                if let Some(gear_i) = has_gear_around(row, col, rows, cols, &grid) {
                    is_current_number_valid = true;
                    gear_index = gear_i;
                }
                current_number = current_number * 10 + c.to_digit(10).unwrap();
            } else {
                if is_current_number_valid {
                    gear_map
                        .entry(gear_index)
                        .and_modify(|v| v.push(current_number))
                        .or_insert(vec![current_number]);
                }
                current_number = 0;
                is_current_number_valid = false;
                gear_index = 0;
            }
        }
    }
    let sum: u32 = gear_map
        .iter()
        .filter(|(_, nums)| nums.len() == 2)
        .map(|(_, nums)| nums[0] * nums[1])
        .sum();
    println!("{sum}");
}

fn has_gear_around(row: i32, col: i32, rows: i32, cols: i32, grid: &Vec<char>) -> Option<i32> {
    for drow in -1..=1 {
        for dcol in -1..=1 {
            let new_row = row + drow;
            let new_col = col + dcol;
            if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
                continue;
            }
            let c = grid[(new_row * rows + new_col) as usize];
            if c == '*' {
                return Some(new_row * rows + new_col);
            }
        }
    }
    None
}
